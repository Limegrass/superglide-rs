use std::{
    fmt::Display,
    time::{Duration, Instant},
};
use termion::event::Event;

#[derive(Debug, Clone)]
pub struct TargetFrameRate {
    #[allow(dead_code)] // kept for Debug fmt
    frame_rate: u32,
    frame_time: Duration,
}

impl From<u32> for TargetFrameRate {
    fn from(value: u32) -> Self {
        TargetFrameRate {
            frame_rate: value,
            frame_time: Duration::new(0, 1_000_000_000 / value),
        }
    }
}

impl TargetFrameRate {
    pub fn elapsed_frames(&self, duration: Duration) -> f64 {
        duration.as_secs_f64() / self.frame_time.as_secs_f64()
    }
}

pub struct Input {
    pub action: Action,
    pub time: Instant,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Crouch(Event),
    Jump(Event),
    Unknown(Event),
}

#[derive(Debug)]
pub struct Percentage(pub f64);

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0 * 100.0)
    }
}

#[derive(Debug)]
pub struct SuperglideAttempt {
    #[allow(dead_code)] // kept for Debug fmt
    chance: Percentage,
    #[allow(dead_code)] // kept for Debug fmt
    frames_elapsed: f64,
}
