pub mod generic;
use generic::{Event, Response, State, StateMachine};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum CallState {
    #[default]
    Idle,
    Dialing,
    Ringing,
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallEvent {
    Dial,
    Incoming,
    Answer,
    HangUp,
    Reset,
}

impl Event for CallEvent {}
impl State for CallState {}

pub fn init_state_machine(ctx: HashMap<String, usize>) -> StateMachine<CallState, CallEvent> {
    let mut sm = StateMachine::new(ctx);

    // Transition from Idle to Dialing on Dial event
    sm.add_transition(CallState::Idle, CallEvent::Dial, |_sm, _event| {
        println!("Transitioning from Idle to Dialing");
        Ok(Response::Transition(CallState::Dialing))
    });

    // Transition from Idle to Ringing on Incoming event
    sm.add_transition(CallState::Idle, CallEvent::Incoming, |_sm, _event| {
        println!("Transitioning from Idle to Ringing");
        Ok(Response::Transition(CallState::Ringing))
    });

    // Transition from Dialing to Connected on Answer event
    sm.add_transition(CallState::Dialing, CallEvent::Answer, |_sm, _event| {
        println!("Transitioning from Dialing to Connected");
        Ok(Response::Transition(CallState::Connected))
    });

    // Transition from Dialing to Disconnected on HangUp event
    sm.add_transition(CallState::Dialing, CallEvent::HangUp, |_sm, _event| {
        println!("Transitioning from Dialing to Disconnected");
        Ok(Response::Transition(CallState::Disconnected))
    });

    // Transition from Ringing to Connected on Answer event
    sm.add_transition(CallState::Ringing, CallEvent::Answer, |_sm, _event| {
        println!("Transitioning from Ringing to Connected");
        Ok(Response::Transition(CallState::Connected))
    });

    // Transition from Ringing to Disconnected on HangUp event
    sm.add_transition(CallState::Ringing, CallEvent::HangUp, |_sm, _event| {
        println!("Transitioning from Ringing to Disconnected");
        Ok(Response::Transition(CallState::Disconnected))
    });

    // Transition from Connected to Disconnected on HangUp event
    sm.add_transition(CallState::Connected, CallEvent::HangUp, |_sm, _event| {
        println!("Transitioning from Connected to Disconnected");
        Ok(Response::Transition(CallState::Disconnected))
    });

    // Transition from any state to Idle on Reset event
    sm.add_transition(CallState::Disconnected, CallEvent::Reset, |_sm, _event| {
        println!("Resetting to Idle");
        Ok(Response::Transition(CallState::Idle))
    });

    sm
}
#[cfg(test)]
mod tests {
    use super::*;
    use generic::{StateMachineError, Stateful};

    #[test]
    fn test_valid_transitions() {
        let ctx = HashMap::<String, usize>::new();
        let mut sm = init_state_machine(ctx);

        assert_eq!(sm.get_current_state().unwrap(), &CallState::Idle);

        sm.handle_event(&CallEvent::Dial)
            .expect("Dial event should succeed");
        assert_eq!(sm.get_current_state().unwrap(), &CallState::Dialing);

        sm.handle_event(&CallEvent::Answer)
            .expect("Answer event should succeed");
        assert_eq!(sm.get_current_state().unwrap(), &CallState::Connected);

        sm.handle_event(&CallEvent::HangUp)
            .expect("HangUp event should succeed");
        assert_eq!(sm.get_current_state().unwrap(), &CallState::Disconnected);
    }

    #[test]
    fn test_invalid_transition() {
        let ctx = HashMap::<String, usize>::new();
        let mut sm = init_state_machine(ctx);

        let result = sm.handle_event(&CallEvent::Answer);
        assert!(matches!(
            result,
            Err(StateMachineError::TransitionNotFound { .. })
        ));
        assert_eq!(sm.get_current_state().unwrap(), &CallState::Idle);
    }
}
