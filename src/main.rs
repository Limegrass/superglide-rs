mod metrics;
mod state;
mod superglide;

use crate::{
    metrics::Metrics,
    state::State,
    superglide::{Action, Input, Percentage, TargetFrameRate},
};
use anyhow::Result;
use clap::Parser;
use std::{
    io::{stdin, Stdin, Stdout, Write},
    time::{Duration, Instant},
};
use termion::{
    event::{Event, Key},
    input::{Events, TermRead},
    raw::{IntoRawMode, RawTerminal},
};

const EXIT_KEY: Event = Event::Key(Key::Ctrl('c'));

fn request_configuration(
    stdout: &mut RawTerminal<Stdout>,
    event_input: &mut Events<Stdin>,
    action_name: &'static str,
) -> Result<Event> {
    write!(stdout, "provide a {} key\r\n", action_name)?;
    let action_key = event_input.next().expect("awaiting user input")?;
    write!(stdout, "{} key: {:?}\r\n", action_name, action_key)?;
    Ok(action_key)
}

#[derive(Parser, Debug)]
struct Args {
    /// Target frame rate you run your game at.
    #[arg(short, long)]
    fps: u32,
    /// Timeout in milliseconds before an atttempt is considered invalid.
    /// This helps automatically reset if you begin crouch and jump in the wrong order.
    /// Defaults to 250ms (1/4 of a second).
    #[arg(short, long, default_value_t = 250)]
    timeout: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("exit with Ctrl + c");

    let mut stdout = std::io::stdout().into_raw_mode()?;
    let target_fps = TargetFrameRate::from(args.fps);
    print!("{:?}\r\n", target_fps);

    let mut input_events = stdin().events();

    // TODO: read from persistent config - requires serializing and deserializing Key
    let jump = request_configuration(&mut stdout, &mut input_events, "jump")?;
    if jump == EXIT_KEY {
        return Ok(());
    }

    let crouch = request_configuration(&mut stdout, &mut input_events, "crouch")?;
    if crouch == EXIT_KEY {
        return Ok(());
    }

    let timeout = Duration::from_millis(args.timeout.into());
    let mut state = State::Idle;
    let mut input_events = termion::async_stdin().events();
    let mut metrics = Metrics::new();

    loop {
        match state {
            State::AwaitCompletion { first_input } if first_input.time.elapsed() > timeout => {
                print!("Too slow, start again from jump\r\n");
                state = State::Idle;
                continue;
            }
            State::Completed {
                first_input,
                second_input,
            } => {
                let elapsed = second_input.time - first_input.time;
                let elapsed_frames = target_fps.elapsed_frames(elapsed);
                print!(
                    "{:?} pressed {:?} ({} frames) after {:?}\r\n",
                    second_input.action, elapsed, elapsed_frames, first_input.action
                );

                // Chance to superglide is how close you are to exactly 1.0
                // If 0 frames passed, 0 chance, if 2 frames, 0 chance
                // If between 0 and 2, some chance
                // 1 - min(abs(elapsed - 1.0))
                let superglide_chance = Percentage(1.0 - (elapsed_frames - 1.0).abs().min(1.0));
                match superglide_chance.0 {
                    chance if chance == 0.0 => {
                        if let Action::Crouch(..) = first_input.action {
                            metrics.record_crouch_first();
                        } else if elapsed_frames > 2.0 {
                            metrics.record_crouch_late();
                        } else {
                            eprint!("you did some weird shit idklol\r\n");
                        }
                    }
                    _ => {
                        metrics.record_possible_superglide(&superglide_chance);
                    }
                }
                print!("superglide chance: {}\r\n", superglide_chance);

                state = State::Idle;
            }
            _ => {}
        }

        for event in &mut input_events {
            let action = match event? {
                event if event == EXIT_KEY => {
                    print!("session stats: {:?}\r\n", metrics);
                    return Ok(());
                }
                event if event == jump => Action::Jump(event),
                event if event == crouch => Action::Crouch(event),
                event => Action::Unknown(event),
            };

            if let Action::Unknown(event) = action {
                print!("invalid key {:?} pressed, resetting\r\n", event);
                state = State::Idle;
                continue;
            }

            match state {
                State::AwaitCompletion { .. } => {
                    state = state.transition_states(Input {
                        action,
                        time: Instant::now(),
                    });
                }
                State::Idle => {
                    state = state.transition_states(Input {
                        action,
                        time: Instant::now(),
                    });
                }
                _ => {}
            };
        }
    }
}
