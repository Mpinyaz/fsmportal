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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallEvent {
    Dial,
    Incoming,
    Answer,
    HangUp,
    Reset,
}

#[derive(Debug)]
pub enum CallError {
    UnexpectedEvent { state: CallState, event: CallEvent },
    TransitionNotFound { from: CallState, event: CallEvent },
}

type Transition<E> = fn(&mut StateMachine, &E) -> Result<(), CallError>;

pub struct StateMachine {
    current_state: CallState,
    context: HashMap<String, usize>,
    transitions: HashMap<(CallState, CallEvent), Transition<CallEvent>>,
}

impl StateMachine {
    pub fn new(context: HashMap<String, usize>) -> Self {
        let mut transitions: HashMap<(CallState, CallEvent), Transition<CallEvent>> =
            HashMap::new();

        transitions.insert((CallState::Idle, CallEvent::Dial), idle_to_dialing);
        transitions.insert((CallState::Idle, CallEvent::Incoming), idle_to_ringing);
        transitions.insert(
            (CallState::Dialing, CallEvent::HangUp),
            dialing_to_disconnected,
        );
        transitions.insert(
            (CallState::Ringing, CallEvent::HangUp),
            ringing_to_disconnected,
        );
        transitions.insert(
            (CallState::Dialing, CallEvent::Answer),
            dialing_to_connected,
        );
        transitions.insert(
            (CallState::Ringing, CallEvent::Answer),
            ringing_to_connected,
        );
        transitions.insert(
            (CallState::Connected, CallEvent::HangUp),
            connected_to_disconnected,
        );
        transitions.insert(
            (CallState::Disconnected, CallEvent::Reset),
            disconnected_to_idle,
        );

        StateMachine {
            current_state: CallState::Idle,
            context,
            transitions,
        }
    }

    pub fn handle_event(&mut self, event: &CallEvent) -> Result<(), CallError> {
        if let Some(&transition) = self
            .transitions
            .get(&(self.current_state.clone(), event.clone()))
        {
            // Call the transition function.
            transition(self, event)
        } else {
            Err(CallError::TransitionNotFound {
                from: self.current_state.clone(),
                event: event.clone(),
            })
        }
    }
}

fn idle_to_dialing(sm: &mut StateMachine, _event: &CallEvent) -> Result<(), CallError> {
    println!("Transitioning from Idle to Dialing");
    sm.current_state = CallState::Dialing;
    Ok(())
}

fn idle_to_ringing(sm: &mut StateMachine, _event: &CallEvent) -> Result<(), CallError> {
    println!("Transitioning from Idle to Ringing");
    sm.current_state = CallState::Ringing;
    Ok(())
}

fn dialing_to_disconnected(sm: &mut StateMachine, _event: &CallEvent) -> Result<(), CallError> {
    println!("Transitioning from Dialing to Disconnected");
    sm.current_state = CallState::Disconnected;
    Ok(())
}

fn ringing_to_disconnected(sm: &mut StateMachine, _event: &CallEvent) -> Result<(), CallError> {
    println!("Transitioning from Ringing to Disconnected");
    sm.current_state = CallState::Disconnected;
    Ok(())
}

fn dialing_to_connected(sm: &mut StateMachine, _event: &CallEvent) -> Result<(), CallError> {
    println!("Transitioning from Dialing to Connected");
    sm.current_state = CallState::Connected;
    Ok(())
}

fn ringing_to_connected(sm: &mut StateMachine, _event: &CallEvent) -> Result<(), CallError> {
    println!("Transitioning from Ringing to Connected");
    sm.current_state = CallState::Connected;
    Ok(())
}

fn connected_to_disconnected(sm: &mut StateMachine, _event: &CallEvent) -> Result<(), CallError> {
    println!("Transitioning from Connected to Disconnected");
    sm.current_state = CallState::Disconnected;
    Ok(())
}

fn disconnected_to_idle(sm: &mut StateMachine, _event: &CallEvent) -> Result<(), CallError> {
    println!("Transitioning from Disconnected to Idle");
    sm.current_state = CallState::Idle;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_context() -> HashMap<String, usize> {
        HashMap::new() // Customize as needed
    }

    #[test]
    fn test_idle_to_dialing() {
        let mut sm = StateMachine::new(setup_context());
        assert_eq!(sm.current_state, CallState::Idle);

        sm.handle_event(&CallEvent::Dial)
            .expect("Failed to transition from Idle to Dialing");
        assert_eq!(sm.current_state, CallState::Dialing);
    }

    #[test]
    fn test_idle_to_ringing() {
        let mut sm = StateMachine::new(setup_context());
        assert_eq!(sm.current_state, CallState::Idle);

        sm.handle_event(&CallEvent::Incoming)
            .expect("Failed to transition from Idle to Ringing");
        assert_eq!(sm.current_state, CallState::Ringing);
    }

    #[test]
    fn test_dialing_to_connected() {
        let mut sm = StateMachine::new(setup_context());
        sm.current_state = CallState::Dialing;

        sm.handle_event(&CallEvent::Answer)
            .expect("Failed to transition from Dialing to Connected");
        assert_eq!(sm.current_state, CallState::Connected);
    }

    #[test]
    fn test_connected_to_disconnected() {
        let mut sm = StateMachine::new(setup_context());
        sm.current_state = CallState::Connected;

        sm.handle_event(&CallEvent::HangUp)
            .expect("Failed to transition from Connected to Disconnected");
        assert_eq!(sm.current_state, CallState::Disconnected);
    }

    #[test]
    fn test_disconnected_to_idle() {
        let mut sm = StateMachine::new(setup_context());
        sm.current_state = CallState::Disconnected;

        sm.handle_event(&CallEvent::Reset)
            .expect("Failed to transition from Disconnected to Idle");
        assert_eq!(sm.current_state, CallState::Idle);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachine::new(setup_context());
        assert_eq!(sm.current_state, CallState::Idle);

        let result = sm.handle_event(&CallEvent::Answer);
        assert!(result.is_err());

        if let Err(CallError::TransitionNotFound { from, event }) = result {
            assert_eq!(from, CallState::Idle);
            assert_eq!(event, CallEvent::Answer);
        } else {
            panic!("Expected TransitionNotFound error, got {:?}", result);
        }
    }
}
