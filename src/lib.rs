use std::collections::HashMap;

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

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            current_state: CallState::Idle, // Default state
            context: HashMap::new(),        // Empty HashMap context
        }
    }
}
