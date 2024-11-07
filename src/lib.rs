use std::collections::HashMap;
use std::fmt::Debug;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallState {
    Idle,
    Dialing,
    Ringing,
    Connected,
    Disconnected,
}

#[derive(Debug)]
pub enum CallEvent {
    Dial,
    Incoming,
    Answer,
    HangUp,
    Reset,
}

#[derive(Debug)]
pub enum Error {
    StateNotFound(String),
    StateMachineNotInitialized,
}
pub type Action<'a, S, E> = Box<dyn Fn(&S, &E) + 'a>;
pub type Predicate<'a, S, E> = Box<dyn Fn(&S, &E) -> bool + 'a>;

pub struct StateMachine {
    current_state: CallState,
    context: HashMap<String, usize>,
}

pub enum Response<S> {
    Handled,
    Transition(S),
}
pub trait StateHandler<S, CTX, E: Debug> {
    fn on_event(&mut self, event: &E, context: &mut CTX) -> Response<S>;
    fn on_exit(&mut self, context: &mut CTX);
}

impl StateMachine {
    pub fn new(context: HashMap<String, usize>) -> Self {
        StateMachine {
            current_state: CallState::Idle,
            context,
        }
    }

    fn transition(&mut self, next_state: CallState) {
        println!(
            "Transitioning from {:?} to {:?}",
            self.current_state, next_state
        );
        self.current_state = next_state;
    }
}
