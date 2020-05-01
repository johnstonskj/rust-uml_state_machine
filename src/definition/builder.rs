/*!
One-line description.

More detailed description, with

# Example

*/

use crate::tag::StateID;
use crate::{ActionFn, ConditionFn, State, StateKind, StateMachine, Transition};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct StateMachineBuilder<E: Eq, D> {
    label: Option<String>,
    states: HashMap<StateID, StateBuilder<E, D>>,
    initial: StateID,
}

pub struct StateBuilder<E: Eq, D> {
    id: StateID,
    label: Option<String>,
    kind: StateKind,
    transitions: Vec<TransitionBuilder<E, D>>,
    parent: Option<StateID>,
    child_states: Vec<StateBuilder<E, D>>,
    on_entry: Vec<ActionFn<D>>,
    body: Vec<ActionFn<D>>,
    on_exit: Vec<ActionFn<D>>,
}

pub struct TransitionBuilder<E: Eq, D> {
    label: Option<String>,
    event: Option<E>,
    target: Option<StateID>,
    internal: bool,
    conditions: Vec<ConditionFn<E, D>>,
    actions: Vec<ActionFn<D>>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<E: Clone + Eq + Hash + Debug, D: Debug> Debug for StateMachineBuilder<E, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateMachineBuilder")
            .field("label", &self.label)
            .field("states", &self.states)
            .field("initial", &self.initial)
            .finish()
    }
}

impl<E: Clone + Eq + Hash, D> Default for StateMachineBuilder<E, D> {
    fn default() -> Self {
        Self {
            label: None,
            states: Default::default(),
            initial: StateID::invalid(),
        }
    }
}

impl<E: Clone + Eq + Hash, D> From<&mut StateMachineBuilder<E, D>> for Rc<StateMachine<E, D>> {
    fn from(builder: &mut StateMachineBuilder<E, D>) -> Self {
        let mut chart: StateMachine<E, D> = StateMachine {
            label: builder.label.clone(),
            states: HashMap::with_capacity(builder.states.len()),
            initial: builder.initial.clone(),
            on_init: vec![],
            on_done: vec![],
        };

        for state in builder.states.values() {
            let _ = chart.states.insert(state.id.clone(), state.build());
        }

        chart.into()
    }
}

impl<E: Clone + Eq + Hash, D> StateMachineBuilder<E, D> {
    fn new() -> Self {
        Default::default()
    }
    pub fn unlabeled(&mut self) -> &mut Self {
        self.label = None;
        self
    }

    pub fn labeled(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn state(&mut self, state: &mut StateBuilder<E, D>) -> &mut Self {
        let _ = self.states.insert(state.id.clone(), state.clone());
        if let StateKind::Initial = state.kind {
            self.initial = state.id.clone();
        }
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl<E: Clone + Eq + Hash, D> Clone for StateBuilder<E, D> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            label: self.label.clone(),
            kind: self.kind.clone(),
            transitions: self.transitions.clone(),
            parent: self.parent.clone(),
            child_states: self.child_states.clone(),
            on_entry: self.on_entry.clone(),
            body: self.body.clone(),
            on_exit: self.on_exit.clone(),
        }
    }
}

impl<E: Clone + Eq + Hash, D> Default for StateBuilder<E, D> {
    fn default() -> Self {
        Self {
            id: StateID::random_with_prefix("state").unwrap(),
            label: None,
            kind: StateKind::Atomic,
            transitions: Default::default(),
            parent: None,
            child_states: Default::default(),
            on_entry: vec![],
            body: vec![],
            on_exit: vec![],
        }
    }
}

impl<E: Clone + Eq + Hash + Debug, D: Debug> Debug for StateBuilder<E, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateBuilder")
            .field("label", &self.label)
            .field("kind", &self.kind)
            .field("transitions", &self.transitions)
            .field("parent", &self.parent)
            .field("child_states", &self.child_states)
            .field("on_entry", &"...".to_string())
            .field("invoke", &"...".to_string())
            .field("on_exit", &"...".to_string())
            .finish()
    }
}

impl<E: Clone + Eq + Hash, D> StateBuilder<E, D> {
    pub fn atomic() -> Self {
        Self::make(StateKind::Atomic, None)
    }

    pub fn atomic_with_id(id: &str) -> Self {
        Self::make(StateKind::Atomic, Some(StateID::from_str(id).unwrap()))
    }

    pub fn compound() -> Self {
        Self::make(
            StateKind::Composite {
                child_states: Default::default(),
                initial: StateID::invalid(),
            },
            None,
        )
    }

    pub fn compound_with_id(id: &str) -> Self {
        Self::make(
            StateKind::Composite {
                child_states: Default::default(),
                initial: StateID::invalid(),
            },
            Some(StateID::from_str(id).unwrap()),
        )
    }

    pub fn parallel() -> Self {
        Self::make(
            StateKind::Orthogonal {
                child_states: Default::default(),
            },
            None,
        )
    }

    pub fn parallel_with_id(id: &str) -> Self {
        Self::make(
            StateKind::Orthogonal {
                child_states: Default::default(),
            },
            Some(StateID::from_str(id).unwrap()),
        )
    }

    pub fn shallow_history() -> Self {
        Self::make(
            StateKind::History {
                deep: false,
                state: vec![],
            },
            None,
        )
    }

    pub fn shallow_history_with_id(id: &str) -> Self {
        Self::make(
            StateKind::History {
                deep: false,
                state: vec![],
            },
            Some(StateID::from_str(id).unwrap()),
        )
    }

    pub fn deep_history() -> Self {
        Self::make(
            StateKind::History {
                deep: true,
                state: vec![],
            },
            None,
        )
    }

    pub fn deep_history_with_id(id: &str) -> Self {
        Self::make(
            StateKind::History {
                deep: true,
                state: vec![],
            },
            Some(StateID::from_str(id).unwrap()),
        )
    }

    pub fn initial() -> Self {
        Self::make(StateKind::Initial, None)
    }

    pub fn initial_with_id(id: &str) -> Self {
        Self::make(StateKind::Initial, Some(StateID::from_str(id).unwrap()))
    }

    pub fn final_state() -> Self {
        Self::make(StateKind::Final, None)
    }

    pub fn final_with_id(id: &str) -> Self {
        Self::make(StateKind::Final, Some(StateID::from_str(id).unwrap()))
    }

    fn make(kind: StateKind, id: Option<StateID>) -> Self {
        Self {
            id: match id {
                None => StateID::random_with_prefix("state").unwrap(),
                Some(id) => id,
            },
            label: None,
            kind,
            transitions: Default::default(),
            parent: None,
            child_states: Default::default(),
            on_entry: Default::default(),
            body: Default::default(),
            on_exit: Default::default(),
        }
    }

    pub fn unlabeled(&mut self) -> &mut Self {
        self.label = None;
        self
    }

    pub fn labeled(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }

    #[inline]
    pub fn and(&mut self) -> &mut Self {
        self
    }

    #[inline]
    pub fn then(&mut self) -> &mut Self {
        self
    }

    #[inline]
    pub fn on_entry(&mut self, action: ActionFn<D>) -> &mut Self {
        self.on_entry.push(action);
        self
    }

    #[inline]
    pub fn action(&mut self, action: ActionFn<D>) -> &mut Self {
        self.body.push(action);
        self
    }

    #[inline]
    pub fn on_exit(&mut self, action: ActionFn<D>) -> &mut Self {
        self.on_exit.push(action);
        self
    }

    pub fn transition(&mut self, transition: &mut TransitionBuilder<E, D>) -> &mut Self {
        if transition.target == Some(StateID::invalid()) {
            transition.target = Some(self.id.clone());
        }
        self.transitions.push(transition.clone());
        self
    }

    pub fn child(&mut self, state: &mut StateBuilder<E, D>) -> &mut Self {
        self.child_states.push(state.clone());
        self
    }

    pub(self) fn build(&self) -> Rc<State<E, D>> {
        let mut state: State<E, D> = State {
            id: self.id.clone(),
            label: self.label.clone(),
            kind: self.kind.clone(),
            transitions: Default::default(),
            parent: None,
            on_entry: self.on_entry.clone(),
            on_run: self.body.clone(),
            on_exit: self.on_exit.clone(),
        };
        for transition in &self.transitions {
            state.transitions.push(transition.build());
        }
        state.into()
    }
}

// ------------------------------------------------------------------------------------------------

impl<E: Clone + Eq + Hash, D> Clone for TransitionBuilder<E, D> {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            event: self.event.clone(),
            target: self.target.clone(),
            internal: self.internal,
            conditions: self.conditions.clone(),
            actions: self.actions.clone(),
        }
    }
}

impl<E: Clone + Eq + Hash + Debug, D: Debug> Debug for TransitionBuilder<E, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransitionBuilder")
            .field("label", &self.label)
            .field("event", &self.event)
            .field("target", &self.target)
            .field("internal", &self.internal)
            .field("conditions", &"...".to_string())
            .field("actions", &"...".to_string())
            .finish()
    }
}

impl<E: Clone + Eq + Hash, D> Default for TransitionBuilder<E, D> {
    fn default() -> Self {
        Self {
            label: None,
            event: None,
            target: None,
            conditions: vec![],
            internal: false,
            actions: vec![],
        }
    }
}

impl<E: Clone + Eq + Hash, D> PartialEq for TransitionBuilder<E, D> {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event && self.target == other.target && self.internal == other.internal
    }
}

impl<E: Clone + Eq + Hash, D> Eq for TransitionBuilder<E, D> {}

impl<E: Clone + Eq + Hash, D> Hash for TransitionBuilder<E, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.event.hash(state);
        self.target.hash(state);
        self.internal.hash(state);
    }
}

impl<E: Clone + Eq + Hash, D> TransitionBuilder<E, D> {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn unlabeled(&mut self) -> &mut Self {
        self.label = None;
        self
    }

    #[inline]
    pub fn labeled(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }

    #[inline]
    pub fn on_event(&mut self, event: E) -> &mut Self {
        self.event = Some(event);
        self
    }

    #[inline]
    pub fn to(&mut self, target_state: &str) -> &mut Self {
        self.target = Some(StateID::from_str(target_state).unwrap());
        self
    }

    #[inline]
    pub fn back_to_self(&mut self) -> &mut Self {
        self.target = Some(StateID::invalid());
        self
    }

    #[inline]
    pub fn if_condition(&mut self, condition: ConditionFn<E, D>) -> &mut Self {
        self.conditions.push(condition);
        self
    }

    #[inline]
    pub fn and(&mut self) -> &mut Self {
        self
    }

    #[inline]
    pub fn then(&mut self) -> &mut Self {
        self
    }

    #[inline]
    pub fn do_action(&mut self, action: ActionFn<D>) -> &mut Self {
        self.actions.push(action);
        self
    }

    #[inline]
    pub fn externally(&mut self) -> &mut Self {
        self.internal = false;
        self
    }

    #[inline]
    pub fn internally(&mut self) -> &mut Self {
        self.internal = true;
        self
    }

    pub(self) fn build(&self) -> Transition<E, D> {
        let transition: Transition<E, D> = Transition {
            label: self.label.clone(),
            event: self.event.clone(),
            target: self.target.clone(),
            internal: self.internal,
            conditions: self.conditions.clone(),
            actions: self.actions.clone(),
        };
        transition
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

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ErrorKind, StateMachineInstance};

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Event {
        This,
        That,
    }

    #[test]
    fn test_simple() {
        let simple: Rc<StateMachine<Event, HashMap<String, String>>> = StateMachineBuilder::new()
            .labeled("simple")
            .state(
                StateBuilder::initial()
                    .labeled("Start Here")
                    .transition(TransitionBuilder::new().to("end")),
            )
            .state(StateBuilder::final_with_id("end").labeled("End Here"))
            .into();

        let valid = simple.validate();
        println!("{:#?}", valid);
        assert!(valid.is_ok());

        let mut instance = StateMachineInstance::new(simple, HashMap::new());

        let result = instance.post(&Event::This);
        assert!(result.is_err());
        match result.err().unwrap().0 {
            ErrorKind::InstanceIsNotActive => println!("error-ed as expected"),
            _ => panic!("expecting ErrorKind::InstanceIsNotActive"),
        }

        let result = instance.execute();
        println!("{:#?}", result);
        println!("{:#?}", instance);
        assert!(result.is_ok());
        assert!(instance.is_done());

        let result = instance.execute();
        assert!(result.is_err());
        match result.err().unwrap().0 {
            ErrorKind::InstanceIsDone => println!("error-ed as expected"),
            _ => panic!("expecting ErrorKind::InstanceIsDone"),
        }
    }
}
