/*!
The core `StateMachine` implementation types.

More detailed description, with

# Example

TBD

*/

use crate::definition::iterators::Actions;
use crate::definition::InternalEvent;
use crate::{ErrorKind, Result, State, StateID, StateKind, StateMachine, Transition};
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::iter::FromIterator;
use std::panic;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct StateMachineInstance<E: Eq, D> {
    id: StateID,
    chart: Rc<StateMachine<E, D>>,
    active: HashSet<StateID>,
    context: RefCell<D>,
    state: RefCell<ExecutionState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum ExecutionState {
    New = 0,
    Active,
    InAction,
    Done,
    #[allow(dead_code)]
    Error,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<E: Clone + Eq + Hash + Debug, D: Debug> Debug for StateMachineInstance<E, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateMachineInstance")
            .field("id", &self.id)
            .field("chart", &self.chart)
            .field("active", &self.active)
            .field("context", &self.context)
            .field("state", &self.state)
            .finish()
    }
}

impl<E: Clone + Eq + Hash, D> StateMachineInstance<E, D> {
    pub fn new(chart: Rc<StateMachine<E, D>>, context: D) -> Self {
        assert!(chart.validate().is_ok());
        Self {
            id: StateID::random_with_prefix("execution").unwrap(),
            chart,
            active: Default::default(),
            context: RefCell::new(context),
            state: RefCell::new(ExecutionState::New),
        }
    }

    pub fn chart(&self) -> Rc<StateMachine<E, D>> {
        self.chart.clone()
    }

    pub fn execute(&mut self) -> Result<()> {
        debug!("StateMachine::execute");
        if self.is_done() {
            Err(ErrorKind::InstanceIsDone.into())
        } else if self.is_active() {
            Err(ErrorKind::InstanceIsActive.into())
        } else if self.state.borrow().is_in_action() {
            Err(ErrorKind::EventDuringAction.into())
        } else {
            let initial_state_id = self.chart.initial_state_id();
            let initial_state = self.chart.get_state(&initial_state_id).unwrap();
            self.post_internal_event(&initial_state_id, None, &InternalEvent::Init);
            self.active = HashSet::from_iter(self.enter_state(initial_state, false).drain(..));
            self.check_done();
            Ok(())
        }
    }

    pub fn accepts(&self) -> HashSet<E, RandomState> {
        self.active_states()
            .map(|id| self.chart.get_state(id).unwrap())
            .map(|state| state.accepts())
            .flatten()
            .collect()
    }

    pub fn post(&mut self, event: &E) -> Result<()> {
        println!("StateMachine::post");
        if self.is_done() {
            Err(ErrorKind::InstanceIsDone.into())
        } else if !self.is_active() {
            Err(ErrorKind::InstanceIsNotActive.into())
        } else if self.state.borrow().is_in_action() {
            Err(ErrorKind::EventDuringAction.into())
        } else {
            // TODO: remove this clone!
            self.active = self
                .active_states()
                .map(|id| self.chart.get_state(id).unwrap())
                .map(|st| self.handle_transition(&st, &Some(event)))
                .flatten()
                .collect::<HashSet<StateID>>();
            self.check_done();
            Ok(())
        }
    }

    pub fn active_states(&self) -> Iter<'_, StateID> {
        self.active.iter()
    }

    pub fn is_active(&self) -> bool {
        self.state.borrow().is_active()
    }

    pub fn is_in_error(&self) -> bool {
        self.state.borrow().is_in_error()
    }

    pub fn is_done(&self) -> bool {
        self.state.borrow().is_done()
    }

    // --------------------------------------------------------------------------------------------

    fn enter_state(&mut self, state: Rc<State<E, D>>, internal: bool) -> Vec<StateID> {
        let state_id = state.id();
        debug!("StateMachine::enter_state({}, .., {}", state_id, internal);
        if !internal {
            self.post_internal_event(&state_id, None, &InternalEvent::Entry);
            self.post_internal_event(&state_id, None, &InternalEvent::Run);
        }
        // The `None` value is used to determine those transitions that require no event
        self.handle_transition(&state, &None)
    }

    fn check_done(&mut self) {
        debug!("StateMachine::check_done");
        let done = !self.active.is_empty()
            && self
                .active
                .iter()
                .map(|id| self.chart.get_state(id).unwrap())
                .all(|st| st.kind() == StateKind::Final);
        if done {
            self.post_internal_event(&StateID::invalid(), None, &InternalEvent::Done);
            let _ = self.state.replace(ExecutionState::Done);
        } else {
            let _ = self.state.replace(ExecutionState::Active);
        }
    }

    fn post_internal_event(
        &self,
        in_state_id: &StateID,
        transition: Option<&Transition<E, D>>,
        on_event: &InternalEvent,
    ) {
        debug!(
            "StateMachine::post_internal_event({}, , {:?})",
            in_state_id, on_event
        );
        let previous_state = self.state.replace(ExecutionState::InAction);
        match on_event {
            InternalEvent::Init => {
                self.run_actions(in_state_id, on_event, self.chart.init_actions());
            }
            InternalEvent::Done => {
                self.run_actions(in_state_id, on_event, self.chart.done_actions());
            }
            InternalEvent::Entry => {
                self.run_actions(
                    in_state_id,
                    on_event,
                    self.chart.get_state(in_state_id).unwrap().entry_actions(),
                );
            }
            InternalEvent::Run => {
                self.run_actions(
                    in_state_id,
                    on_event,
                    self.chart.get_state(in_state_id).unwrap().body_actions(),
                );
            }
            InternalEvent::Exit => {
                self.run_actions(
                    in_state_id,
                    on_event,
                    self.chart.get_state(in_state_id).unwrap().exit_actions(),
                );
            }
            InternalEvent::Transition => {
                self.run_actions(in_state_id, on_event, transition.unwrap().actions());
            }
        }
        let _ = self.state.replace(previous_state);
    }

    fn run_actions(
        &self,
        in_state_id: &StateID,
        on_event: &InternalEvent,
        actions: Actions<'_, D>,
    ) {
        for action in actions {
            action(in_state_id, on_event, &mut self.context.borrow_mut());
        }
    }

    fn handle_transition(
        &self,
        from_state: &Rc<State<E, D>>,
        on_event: &Option<&E>,
    ) -> Vec<StateID> {
        debug!(
            "StateMachine::handle_transition is_some={}",
            on_event.is_some()
        );
        // Find all transitions that handle this event
        let transitions = from_state
            .transitions()
            .filter(|t| t.event() == on_event.map(|e| e.clone()))
            .collect::<Vec<_>>();
        trace!(
            "StateMachine::handle_transition > enabled transitions={}",
            transitions.len()
        );
        if !transitions.is_empty() {
            if transitions.iter().any(|t| !t.is_internal()) {
                self.post_internal_event(&from_state.id(), None, &InternalEvent::Exit);
            }
            trace!("StateMachine::handle_transition > testing all outbound transitions");
            transitions
                .iter()
                .filter_map(|t| self.fire_state_transitions(&from_state, t, on_event))
                .collect()
        } else {
            Vec::default()
        }
    }

    fn fire_state_transitions(
        &self,
        from_state: &Rc<State<E, D>>,
        transition: &Transition<E, D>,
        on_event: &Option<&E>,
    ) -> Option<StateID> {
        debug!("StateMachine::fire_state_transitions");
        if on_event.map(|e| e.clone()) == transition.event() {
            trace!("StateMachine::fire_state_transitions > event matches");
            if transition
                .conditions()
                .all(|c| c(&from_state.id(), &on_event, &self.context.borrow()))
            {
                self.post_internal_event(
                    &from_state.id(),
                    Some(transition),
                    &InternalEvent::Transition,
                );
                transition.target_state_id()
            } else {
                trace!("StateMachine::fire_state_transitions > not all conditions met");
                None
            }
        } else {
            trace!("StateMachine::fire_state_transitions > event does not match");
            None
        }
    }
}

impl ExecutionState {
    #[allow(dead_code)]
    fn is_new(&self) -> bool {
        match self {
            ExecutionState::New => true,
            _ => false,
        }
    }

    fn is_active(&self) -> bool {
        match self {
            ExecutionState::Active => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_in_action(&self) -> bool {
        match self {
            ExecutionState::InAction => true,
            _ => false,
        }
    }

    fn is_in_error(&self) -> bool {
        match self {
            ExecutionState::Error => true,
            _ => false,
        }
    }

    fn is_done(&self) -> bool {
        match self {
            ExecutionState::Done => true,
            _ => false,
        }
    }
}
// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
