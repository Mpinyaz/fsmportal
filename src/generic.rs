use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

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

pub trait Stateful<S: Clone + Debug + Eq + Hash, CTX, E: Debug> {
    fn on_enter(&self, event: &E);
    fn handle_event(&mut self, event: &E) -> Result<Response<S>, StateMachineError<S, E>>;
    fn on_exit(&self);
}
pub type TransitionFunction<S, E, C> = Arc<
    dyn Fn(&mut StateMachine<S, E, C>, &E) -> Result<Response<S>, StateMachineError<S, E>>
        + Send
        + Sync,
>;
pub struct StateMachine<S, E, C = HashMap<String, usize>>
where
    S: State,
    E: Event,
{
    current_state: S,
    context: C,
    transitions: HashMap<(S, E), TransitionFunction<S, E, C>>,
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
            + 'static
            + Send
            + Sync,
    {
        self.transitions.insert((from, event), Arc::new(transition));
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
impl<S, E, C> Stateful<S, C, E> for StateMachine<S, E, C>
where
    S: State,
    E: Event,
{
    fn on_enter(&self, event: &E) {
        println!("Transition initiated, Call Event: {:?} triggered", event);
    }

    fn handle_event(&mut self, event: &E) -> Result<Response<S>, StateMachineError<S, E>> {
        let current_state = self.current_state.clone();
        let event_clone = event.clone();

        let transition = match self
            .transitions
            .get(&(current_state.clone(), event_clone.clone()))
        {
            Some(t) => t.clone(),
            None => {
                return Err(StateMachineError::TransitionNotFound {
                    from: current_state,
                    event: event_clone,
                })
            }
        };

        self.on_exit();

        match transition(self, event)? {
            Response::Handled => Ok(Response::Handled),
            Response::Transition(new_state) => {
                self.current_state = new_state.clone();
                self.on_enter(event);
                Ok(Response::Transition(new_state))
            }
            Response::Super => Err(StateMachineError::UnexpectedEvent {
                state: current_state,
                event: event_clone,
            }),
        }
    }

    fn on_exit(&self) {
        println!("Exiting state: {:?}", self.current_state);
    }
}
