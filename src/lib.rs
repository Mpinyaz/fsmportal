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
    StateMachineNotInitialized,
}

// Updated type signature to work directly with StateMachine
type Transition<'a, E> = Box<dyn Fn(&mut StateMachine, &E) -> Result<(), CallError> + 'a>;
type Transitions<'a, E> = HashMap<(&'a CallState, &'a E), Transition<'a, E>>;

pub struct StateMachine {
    current_state: CallState,
    context: HashMap<String, usize>,
    transitions: Transitions<'static, CallEvent>,
}

impl StateMachine {
    pub fn new(context: HashMap<String, usize>) -> Self {
        let mut transitions = Transitions::new();
        transitions.insert(
            (&CallState::Idle, &CallEvent::Dial),
            Box::new(Self::idle_to_dialing),
        );
        transitions.insert(
            (&CallState::Idle, &CallEvent::Incoming),
            Box::new(Self::idle_to_ringing),
        );
        transitions.insert(
            (&CallState::Dialing, &CallEvent::HangUp),
            Box::new(Self::dialing_to_disconnected),
        );
        transitions.insert(
            (&CallState::Ringing, &CallEvent::HangUp),
            Box::new(Self::ringing_to_disconnected),
        );
        transitions.insert(
            (&CallState::Dialing, &CallEvent::Answer),
            Box::new(Self::dialing_to_connected),
        );
        transitions.insert(
            (&CallState::Ringing, &CallEvent::Answer),
            Box::new(Self::ringing_to_connected),
        );
        transitions.insert(
            (&CallState::Connected, &CallEvent::HangUp),
            Box::new(Self::connected_to_disconnected),
        );
        transitions.insert(
            (&CallState::Disconnected, &CallEvent::Reset),
            Box::new(Self::disconnected_to_idle),
        );

        StateMachine {
            current_state: CallState::Idle,
            context,
            transitions,
        }
    }

    pub fn handle_event(&mut self, event: &CallEvent) -> Result<(), CallError> {
        // Look for a matching transition based on current_state and event
        if let Some(transition) = self.transitions.get(&(&self.current_state.clone(), event)) {
            transition(self, event)
        } else {
            Err(CallError::TransitionNotFound {
                from: self.current_state.clone(),
                event: event.clone(),
            })
        }
    }

    fn idle_to_dialing(&mut self, _event: &CallEvent) -> Result<(), CallError> {
        println!("Transitioning from Idle to Dialing");
        self.current_state = CallState::Dialing;
        Ok(())
    }

    fn idle_to_ringing(&mut self, _event: &CallEvent) -> Result<(), CallError> {
        println!("Transitioning from Idle to Ringing");
        self.current_state = CallState::Ringing;
        Ok(())
    }

    fn dialing_to_disconnected(&mut self, _event: &CallEvent) -> Result<(), CallError> {
        println!("Transitioning from Dialing to Disconnected");
        self.current_state = CallState::Disconnected;
        Ok(())
    }

    fn ringing_to_disconnected(&mut self, _event: &CallEvent) -> Result<(), CallError> {
        println!("Transitioning from Ringing to Disconnected");
        self.current_state = CallState::Disconnected;
        Ok(())
    }

    fn dialing_to_connected(&mut self, _event: &CallEvent) -> Result<(), CallError> {
        println!("Transitioning from Dialing to Connected");
        self.current_state = CallState::Connected;
        Ok(())
    }

    fn ringing_to_connected(&mut self, _event: &CallEvent) -> Result<(), CallError> {
        println!("Transitioning from Ringing to Connected");
        self.current_state = CallState::Connected;
        Ok(())
    }

    fn connected_to_disconnected(&mut self, _event: &CallEvent) -> Result<(), CallError> {
        println!("Transitioning from Connected to Disconnected");
        self.current_state = CallState::Disconnected;
        Ok(())
    }

    fn disconnected_to_idle(&mut self, _event: &CallEvent) -> Result<(), CallError> {
        println!("Transitioning from Disconnected to Idle");
        self.current_state = CallState::Idle;
        Ok(())
    }
}
