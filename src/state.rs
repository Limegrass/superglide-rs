use crate::superglide::Input;

pub enum State {
    Idle,
    AwaitCompletion {
        first_input: Input,
    },
    Completed {
        first_input: Input,
        second_input: Input,
    },
}

impl State {
    pub fn transition_states(self, event: Input) -> State {
        match self {
            State::Idle => State::AwaitCompletion { first_input: event },
            State::AwaitCompletion {
                first_input: first_action,
            } => State::Completed {
                first_input: first_action,
                second_input: event,
            },
            State::Completed { .. } => State::Idle,
        }
    }
}
