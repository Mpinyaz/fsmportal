mod generic;
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

pub enum Response<S> {
    Handled,
    Super,
    Transition(S),
}

impl<S> Debug for Response<S>
where
    S: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Handled => f.debug_tuple("Handled").finish(),
            Self::Super => f.debug_tuple("Super").finish(),
            Self::Transition(state) => f
                .debug_tuple("Transition")
                .field(state as &dyn Debug)
                .finish(),
        }
    }
}

type Transition = fn(&mut StateMachine, &CallEvent) -> Result<Response<CallState>, CallError>;

pub struct StateMachine {
    current_state: CallState,
    context: HashMap<String, usize>,
    transitions: HashMap<(CallState, CallEvent), Transition>,
}

fn idle_to_dialing(
    _sm: &mut StateMachine,
    _event: &CallEvent,
) -> Result<Response<CallState>, CallError> {
    println!(
        "Transitioning from {:?} to {:?}",
        CallState::Idle,
        CallState::Dialing
    );
    Ok(Response::Transition(CallState::Dialing))
}

fn idle_to_ringing(
    _sm: &mut StateMachine,
    _event: &CallEvent,
) -> Result<Response<CallState>, CallError> {
    println!(
        "Transitioning from {:?} to {:?}",
        CallState::Idle,
        CallState::Ringing
    );
    Ok(Response::Transition(CallState::Ringing))
}

fn dialing_to_disconnected(
    _sm: &mut StateMachine,
    _event: &CallEvent,
) -> Result<Response<CallState>, CallError> {
    println!(
        "Transitioning from {:?} to {:?}",
        CallState::Dialing,
        CallState::Disconnected
    );
    Ok(Response::Transition(CallState::Disconnected))
}

fn ringing_to_disconnected(
    _sm: &mut StateMachine,
    _event: &CallEvent,
) -> Result<Response<CallState>, CallError> {
    println!(
        "Transitioning from {:?} to {:?}",
        CallState::Ringing,
        CallState::Disconnected
    );
    Ok(Response::Transition(CallState::Disconnected))
}

fn dialing_to_connected(
    _sm: &mut StateMachine,
    _event: &CallEvent,
) -> Result<Response<CallState>, CallError> {
    println!(
        "Transitioning from {:?} to {:?}",
        CallState::Dialing,
        CallState::Connected
    );
    Ok(Response::Transition(CallState::Connected))
}

fn ringing_to_connected(
    _sm: &mut StateMachine,
    _event: &CallEvent,
) -> Result<Response<CallState>, CallError> {
    println!(
        "Transitioning from {:?} to {:?}",
        CallState::Ringing,
        CallState::Connected
    );
    Ok(Response::Transition(CallState::Connected))
}

fn connected_to_disconnected(
    _sm: &mut StateMachine,
    _event: &CallEvent,
) -> Result<Response<CallState>, CallError> {
    println!(
        "Transitioning from {:?} to {:?}",
        CallState::Connected,
        CallState::Disconnected
    );
    Ok(Response::Transition(CallState::Disconnected))
}

fn disconnected_to_idle(
    _sm: &mut StateMachine,
    _event: &CallEvent,
) -> Result<Response<CallState>, CallError> {
    println!(
        "Transitioning from {:?} to {:?}",
        CallState::Disconnected,
        CallState::Idle
    );
    Ok(Response::Transition(CallState::Idle))
}

impl StateMachine {
    pub fn new(context: HashMap<String, usize>) -> Self {
        let mut transitions: HashMap<(CallState, CallEvent), Transition> = HashMap::new();

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
        let current_state = self.current_state.clone();
        if let Some(&transition) = self
            .transitions
            .get(&(current_state.clone(), event.clone()))
        {
            match transition(self, event)? {
                Response::Handled => Ok(()),
                Response::Transition(new_state) => {
                    self.current_state = new_state;
                    Ok(())
                }
                Response::Super => Err(CallError::UnexpectedEvent {
                    state: self.current_state.clone(),
                    event: event.clone(),
                }),
            }
        } else {
            Err(CallError::TransitionNotFound {
                from: self.current_state.clone(),
                event: event.clone(),
            })
        }
    }

    pub fn get_current_state(&self) -> &CallState {
        &self.current_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_context() -> HashMap<String, usize> {
        HashMap::new()
    }

    #[test]
    fn test_idle_to_dialing() {
        let mut sm = StateMachine::new(setup_context());
        assert_eq!(sm.get_current_state(), &CallState::Idle);

        sm.handle_event(&CallEvent::Dial)
            .expect("Failed to transition from Idle to Dialing");
        assert_eq!(sm.get_current_state(), &CallState::Dialing);
    }

    #[test]
    fn test_idle_to_ringing() {
        let mut sm = StateMachine::new(setup_context());
        assert_eq!(sm.get_current_state(), &CallState::Idle);

        sm.handle_event(&CallEvent::Incoming)
            .expect("Failed to transition from Idle to Ringing");
        assert_eq!(sm.get_current_state(), &CallState::Ringing);
    }

    #[test]
    fn test_dialing_to_connected() {
        let mut sm = StateMachine::new(setup_context());
        sm.current_state = CallState::Dialing;

        sm.handle_event(&CallEvent::Answer)
            .expect("Failed to transition from Dialing to Connected");
        assert_eq!(sm.get_current_state(), &CallState::Connected);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachine::new(setup_context());
        assert_eq!(sm.get_current_state(), &CallState::Idle);

        let result = sm.handle_event(&CallEvent::Answer);
        assert!(result.is_err());

        if let Err(CallError::TransitionNotFound { from, event }) = result {
            assert_eq!(from, CallState::Idle);
            assert_eq!(event, CallEvent::Answer);
        } else {
            panic!("Expected TransitionNotFound error");
        }
    }
}
