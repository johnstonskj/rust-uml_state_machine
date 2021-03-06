// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

/*
impl<E: Clone + Eq + Hash + Debug, D: Debug> Debug for StateMachine<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateMachine")
            .field("label", &self.label)
            .field("states", &self.states)
            .field("initial", &self.initial)
            .field("on_init", &format!("[..{}]", self.on_init.len()))
            .field("on_done", &format!("[..{}]", self.on_done.len()))
            .finish()
    }
}

impl<E: Clone + Eq + Hash> Default for StateMachine<E> {
    fn default() -> Self {
        Self {
            label: None,
            states: Default::default(),
            initial: StateID::invalid(),
            on_init: Default::default(),
            on_done: Default::default(),
        }
    }
}

impl<E: Clone + Eq + Hash> StateMachine<E> {
    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    pub fn initial_state_id(&self) -> StateID {
        self.initial.clone()
    }

    pub fn accepts(&self) -> HashSet<E, RandomState> {
        self.states
            .values()
            .map(|state| state.accepts())
            .flatten()
            .collect()
    }

    pub fn has_state(&self, id: &StateID) -> bool {
        self.states.contains_key(id)
    }

    pub fn get_state(&self, id: &StateID) -> Option<Rc<State<E>>> {
        self.states.get(id).cloned()
    }

    pub fn add_state(&mut self, state: Rc<State<E>>) {
        let _ = self.states.insert(state.id(), state);
    }

    pub fn has_init_actions(&self) -> bool {
        !self.on_init.is_empty()
    }

    pub fn init_actions(&self) -> iterators::Actions<'_> {
        self.on_init.iter()
    }

    pub fn has_done_actions(&self) -> bool {
        !self.on_done.is_empty()
    }

    pub fn done_actions(&self) -> iterators::Actions<'_> {
        self.on_done.iter()
    }

    pub fn validate(&self) -> Result<()> {
        fn final_count<E: Clone + Eq + Hash>(count: i32, st: &Rc<State<E>>) -> i32 {
            if st.kind() == StateKind::Final {
                count + 1
            } else {
                count
            }
        }

        if self.states.is_empty() {
            return Err(ErrorKind::ChartStatesEmpty.into());
        }
        match self.get_state(&self.initial) {
            None => {
                return Err(ErrorKind::ChartInvalidInitialStateName.into());
            }
            Some(state) => {
                if state.kind != StateKind::Initial {
                    return Err(ErrorKind::ChartInvalidInitialStateKind.into());
                }
            }
        }
        if self.states.values().fold(0, final_count) == 0 {
            return Err(ErrorKind::ChartNoFinalState.into());
        }

        let result: std::result::Result<Vec<()>, Error> =
            self.states.values().map(|st| st.validate(self)).collect();
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for StateKind {
    fn default() -> Self {
        Self::Atomic
    }
}

// ------------------------------------------------------------------------------------------------

impl<E: Clone + Eq + Hash> PartialEq for State<E> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<E: Clone + Eq + Hash> Eq for State<E> {}

impl<E: Clone + Eq + Hash> Hash for State<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<E: Clone + Eq + Hash + Debug, D: Debug> Debug for State<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("kind", &self.kind)
            .field("transitions", &self.transitions)
            .field("parent", &self.parent)
            .field("on_entry", &format!("[..{}]", self.on_entry.len()))
            .field("body", &format!("[..{}]", self.do_activity.len()))
            .field("on_exit", &format!("[..{}]", self.on_exit.len()))
            .finish()
    }
}

impl<E: Clone + Eq + Hash> State<E> {
    pub fn id(&self) -> StateID {
        self.id.clone()
    }

    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    pub fn kind(&self) -> StateKind {
        self.kind.clone()
    }

    pub fn accepts(&self) -> HashSet<E, RandomState> {
        self.transitions
            .iter()
            .filter_map(|t| t.event.clone())
            .collect()
    }

    pub fn has_transitions(&self) -> bool {
        !self.transitions.is_empty()
    }

    pub fn transitions(&self) -> iterators::Transitions<'_, E> {
        self.transitions.iter()
    }

    pub fn add_transition(&mut self, transition: Transition<E>) {
        self.transitions.push(transition);
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn parent_state_id(&self) -> Option<StateID> {
        self.parent.clone()
    }

    pub fn has_children(&self) -> bool {
        match &self.kind {
            StateKind::Composite { child_states, .. } => !child_states.is_empty(),
            StateKind::Orthogonal { child_states } => !child_states.is_empty(),
            _ => false,
        }
    }

    pub fn child_state_ids(&self) -> Option<iterators::StateIDs<'_>> {
        match &self.kind {
            StateKind::Composite { child_states, .. } | StateKind::Orthogonal { child_states } => {
                Some(child_states.iter())
            }
            _ => None,
        }
    }

    pub fn initial_child_id(&self) -> Option<StateID> {
        match &self.kind {
            StateKind::Composite { initial, .. } => Some(initial.clone()),
            _ => None,
        }
    }

    pub fn add_state(&mut self, state: Rc<State<E>>, chart: &mut StateMachine<E>) {
        match &mut self.kind {
            StateKind::Composite { child_states, .. } => {
                let id = state.id();
                chart.add_state(state);
                child_states.push(id);
            }
            StateKind::Orthogonal { child_states } => {
                let id = state.id();
                chart.add_state(state);
                child_states.push(id);
            }
            _ => (),
        }
    }

    pub fn has_entry_actions(&self) -> bool {
        !self.on_entry.is_empty()
    }

    pub fn entry_actions(&self) -> iterators::Actions<'_> {
        self.on_entry.iter()
    }

    pub fn has_body_actions(&self) -> bool {
        !self.do_activity.is_empty()
    }

    pub fn body_actions(&self) -> iterators::Actions<'_> {
        self.do_activity.iter()
    }

    pub fn has_exit_actions(&self) -> bool {
        !self.on_exit.is_empty()
    }

    pub fn exit_actions(&self) -> iterators::Actions<'_> {
        self.on_exit.iter()
    }

    pub(self) fn validate(&self, chart: &StateMachine<E>) -> Result<()> {
        match &self.kind {
            StateKind::Atomic => {}
            StateKind::Composite {
                child_states,
                initial,
            } => {
                if child_states.is_empty() {
                    return Err(ErrorKind::StateChildStatesEmpty.into());
                }
                match chart.get_state(initial) {
                    None => {
                        return Err(ErrorKind::StateInitialState.into());
                    }
                    Some(state) => {
                        if state.kind != StateKind::Initial {
                            return Err(ErrorKind::StateInitialState.into());
                        }
                    }
                }
            }
            StateKind::Orthogonal { child_states } => {
                if child_states.is_empty() {
                    return Err(ErrorKind::StateChildStatesEmpty.into());
                }
            }
            StateKind::History { .. } => {}
            StateKind::Initial => {}
            StateKind::Final => {
                if !self.transitions.is_empty() {
                    return Err(ErrorKind::FinalStateTransitions.into());
                }
            }
        }
        let result: std::result::Result<Vec<()>, Error> = self
            .transitions
            .iter()
            .map(|st| st.validate(chart))
            .collect();
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<E: Clone + Eq + Hash + Debug, D: Debug> Debug for Transition<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transition")
            .field("event", &self.event)
            .field("target", &self.target)
            .field("internal", &self.internal)
            .field("conditions", &format!("[..{}]", self.guard.len()))
            .field("actions", &format!("[..{}]", self.actions.len()))
            .finish()
    }
}

impl<E: Clone + Eq + Hash> PartialEq for Transition<E> {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event && self.target == other.target && self.internal == other.internal
    }
}

impl<E: Clone + Eq + Hash> Eq for Transition<E> {}

impl<E: Clone + Eq + Hash> Hash for Transition<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.event.hash(state);
        self.target.hash(state);
        self.internal.hash(state);
    }
}

impl<E: Clone + Eq + Hash> Transition<E> {
    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    pub fn event(&self) -> Option<E> {
        self.event.clone()
    }

    pub fn target_state_id(&self) -> Option<StateID> {
        self.target.clone()
    }

    pub fn is_internal(&self) -> bool {
        self.internal
    }

    pub fn is_conditional(&self) -> bool {
        !self.guard.is_empty()
    }

    pub fn conditions(&self) -> iterators::Conditions<'_, E> {
        self.guard.iter()
    }

    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    pub fn actions(&self) -> iterators::Actions<'_> {
        self.actions.iter()
    }

    pub(self) fn validate(&self, chart: &StateMachine<E>) -> Result<()> {
        if self.event.is_none() && self.target.is_none() && self.guard.is_empty() {
            return Err(ErrorKind::TransitionTrigger.into());
        }
        if let Some(target) = &self.target {
            if !chart.has_state(&target) {
                return Err(ErrorKind::TransitionTargetState.into());
            }
        }
        Ok(())
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

//pub mod behavior;

pub mod builder;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StateMachineInstance;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Event {
        This,
        That,
    }

    #[test]
    fn test_invalid_no_states() {
        let chart: StateMachine<Event, HashMap<String, String>> = StateMachine::default();

        let result = chart.validate();
        assert!(result.is_err());
        match result.err().unwrap().0 {
            ErrorKind::ChartStatesEmpty => println!("error-ed as expected"),
            _ => panic!("expecting ErrorKind::ChartStatesEmpty"),
        }
    }

    #[test]
    fn test_invalid_no_initial_state() {
        let mut chart = StateMachine::default();
        let state: State<Event, HashMap<String, String>> = State {
            id: StateID::from_str("a-state").unwrap(),
            label: None,
            kind: StateKind::Final,
            transitions: Default::default(),
            parent: None,
            on_entry: Default::default(),
            do_activity: Default::default(),
            on_exit: Default::default(),
        };
        chart.add_state(Rc::new(state));

        let result = chart.validate();
        assert!(result.is_err());
        match result.err().unwrap().0 {
            ErrorKind::ChartInvalidInitialStateName => println!("error-ed as expected"),
            _ => panic!("expecting ErrorKind::ChartInvalidInitialStateName"),
        }
    }

    #[test]
    fn test_invalid_wrong_initial_state_type() {
        let mut chart = StateMachine::default();
        let state: State<Event, HashMap<String, String>> = State {
            id: StateID::from_str("a-state").unwrap(),
            label: None,
            kind: StateKind::Final,
            transitions: Default::default(),
            parent: None,
            on_entry: Default::default(),
            do_activity: Default::default(),
            on_exit: Default::default(),
        };
        chart.initial = StateID::from_str("a-state").unwrap();
        chart.add_state(Rc::new(state));

        let result = chart.validate();
        assert!(result.is_err());
        match result.err().unwrap().0 {
            ErrorKind::ChartInvalidInitialStateKind => println!("error-ed as expected"),
            _ => panic!("expecting ErrorKind::ChartInvalidInitialStateKind"),
        }
    }

    #[test]
    fn test_invalid_wrong_initial_state_name() {
        let mut chart = StateMachine::default();
        let state: State<Event, HashMap<String, String>> = State {
            id: StateID::from_str("a-state").unwrap(),
            label: None,
            kind: StateKind::Final,
            transitions: Default::default(),
            parent: None,
            on_entry: Default::default(),
            do_activity: Default::default(),
            on_exit: Default::default(),
        };
        chart.initial = StateID::from_str("another-state").unwrap();
        chart.add_state(Rc::new(state));

        let result = chart.validate();
        assert!(result.is_err());
        match result.err().unwrap().0 {
            ErrorKind::ChartInvalidInitialStateName => println!("error-ed as expected"),
            _ => panic!("expecting ErrorKind::ChartInvalidInitialStateName"),
        }
    }

    #[test]
    fn test_invalid_no_final_state() {
        let mut chart = StateMachine::default();
        let state: State<Event, HashMap<String, String>> = State {
            id: StateID::from_str("a-state").unwrap(),
            label: None,
            kind: StateKind::Initial,
            transitions: Default::default(),
            parent: None,
            on_entry: Default::default(),
            do_activity: Default::default(),
            on_exit: Default::default(),
        };
        chart.initial = StateID::from_str("a-state").unwrap();
        chart.add_state(Rc::new(state));

        let result = chart.validate();
        assert!(result.is_err());
        match result.err().unwrap().0 {
            ErrorKind::ChartNoFinalState => println!("error-ed as expected"),
            _ => panic!("expecting ErrorKind::ChartNoFinalState"),
        }
    }

    #[test]
    fn test_simple_machine() {
        let mut simple: StateMachine<Event, HashMap<String, String>> = StateMachine {
            label: Some("simple".to_string()),
            states: Default::default(),
            initial: StateID::from_str("init").unwrap(),
            on_init: vec![],
            on_done: vec![],
        };

        let transition: Transition<Event, HashMap<String, String>> = Transition {
            label: Some("via".to_string()),
            event: None,
            target: Some(StateID::from_str("end").unwrap()),
            guard: vec![],
            internal: false,
            actions: vec![],
        };

        let mut init: State<Event, HashMap<String, String>> = State {
            id: StateID::from_str("init").unwrap(),
            label: Some("Start Here".to_string()),
            kind: StateKind::Initial,
            transitions: Default::default(),
            on_entry: Default::default(),
            do_activity: Default::default(),
            on_exit: Default::default(),
            parent: None,
        };
        init.add_transition(transition);

        let end: State<Event, HashMap<String, String>> = State {
            id: StateID::from_str("end").unwrap(),
            label: Some("End Here".to_string()),
            kind: StateKind::Final,
            transitions: Default::default(),
            parent: None,
            on_entry: Default::default(),
            do_activity: Default::default(),
            on_exit: Default::default(),
        };

        simple.add_state(Rc::new(init));
        simple.add_state(Rc::new(end));

        println!("{:#?}", simple);
        println!("{:#?}", simple.validate());
        assert!(simple.validate().is_ok());

        let mut instance = StateMachineInstance::new(simple.into(), HashMap::new());
        let _ = instance.execute();
        println!("{:#?}", instance);
        assert!(instance.is_done());

        let result = instance.execute();
        assert!(result.is_err());
        match result.err().unwrap().0 {
            ErrorKind::InstanceIsDone => println!("error-ed as expected"),
            _ => panic!("expecting ErrorKind::InstanceIsDone"),
        }
    }
}

*/

pub mod id;

pub mod types;

#[doc(hidden)]
pub mod impls;
