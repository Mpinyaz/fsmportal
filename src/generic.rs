use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait State: Clone + Debug + Eq + Hash {}

pub trait Event: Clone + Debug + Eq + Hash {}

#[derive(Debug)]
pub enum StateMachineError<S, E> {
    UnexpectedEvent { state: S, event: E },
    TransitionNotFound { from: S, event: E },
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

pub struct StateMachine<S, E, C = HashMap<String, usize>>
where
    S: State,
    E: Event,
{
    current_state: S,
    context: C,
    transitions: HashMap<
        (S, E),
        Box<dyn Fn(&mut StateMachine<S, E, C>, &E) -> Result<Response<S>, StateMachineError<S, E>>>,
    >,
}

impl<S, E, C> StateMachine<S, E, C>
where
    S: State,
    E: Event,
{
    pub fn new(initial_state: S, context: C) -> Self {
        StateMachine {
            current_state: initial_state,
            context,
            transitions: HashMap::new(),
        }
    }

    pub fn add_transition<F>(&mut self, from: S, event: E, transition: F)
    where
        F: Fn(&mut StateMachine<S, E, C>, &E) -> Result<Response<S>, StateMachineError<S, E>>
            + 'static,
    {
        self.transitions.insert((from, event), Box::new(transition));
    }

    pub fn handle_event(&mut self, event: &E) -> Result<(), StateMachineError<S, E>> {
        let current_state = self.current_state.clone();
        let event_clone = event.clone();

        // Find the transition function
        let transition = self
            .transitions
            .get(&(current_state.clone(), event_clone.clone()))
            .cloned();

        // Early return if no transition is found
        let transition = match transition {
            Some(t) => t,
            None => {
                return Err(StateMachineError::TransitionNotFound {
                    from: current_state,
                    event: event_clone,
                })
            }
        };

        // Execute the transition
        match transition(self, event)? {
            Response::Handled => Ok(()),
            Response::Transition(new_state) => {
                self.current_state = new_state;
                Ok(())
            }
            Response::Super => Err(StateMachineError::UnexpectedEvent {
                state: current_state,
                event: event_clone,
            }),
        }
    }

    pub fn get_current_state(&self) -> &S {
        &self.current_state
    }

    pub fn get_context(&self) -> &C {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut C {
        &mut self.context
    }
}
