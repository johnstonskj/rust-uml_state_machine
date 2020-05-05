/*!
One-line description.

More detailed description, with

# Example

*/

use crate::definition::id::ID;
use crate::definition::types::*;
use crate::error::Result;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! make_labeled_impl {
    ($type_name:ident) => {
        impl Labeled for $type_name {
            fn label(&self) -> &Option<String> {
                &self.label
            }

            fn set_label(&mut self, label: &str) {
                self.label = Some(label.to_string())
            }

            fn unset_label(&mut self) {
                self.label = None
            }
        }
    };
    ($type_name:ident, $type_param:ident) => {
        impl<$type_param: PartialEq> Labeled for $type_name<$type_param> {
            fn label(&self) -> &Option<String> {
                &self.label
            }

            fn set_label(&mut self, label: &str) {
                self.label = Some(label.to_string())
            }

            fn unset_label(&mut self) {
                self.label = None
            }
        }
    };
}

macro_rules! make_identified_impl {
    ($type_name:ident) => {
        impl Identified for $type_name {
            fn id(&self) -> &ID {
                &self.id
            }
        }
    };
    ($type_name:ident, $type_param:ident) => {
        impl<$type_param: PartialEq> Identified for $type_name<$type_param> {
            fn id(&self) -> &ID {
                &self.id
            }
        }
    };
}

macro_rules! make_has_regions_impl {
    ($type_name:ident) => {
        impl<'a, E: PartialEq> HasRegions<'a, E> for $type_name<E> {
            fn has_regions(&self) -> bool {
                !self.regions.is_empty()
            }

            fn regions(&self) -> Iter<'_, Region<E>> {
                self.regions.iter()
            }

            fn region(&self, index: usize) -> Option<&Region<E>> {
                self.regions.get(index)
            }

            fn add_region(&mut self, region: Region<E>) {
                self.regions.push(region)
            }
        }
    };
}

macro_rules! make_contained_impl {
    ($type_name:ident) => {
        impl Contained for $type_name {
            fn container(&self) -> &ID {
                &self.container
            }

            fn set_container(&mut self, container: ID) {
                self.container = container
            }
        }
    };
    ($type_name:ident, $type_param:ident) => {
        impl<$type_param: PartialEq> Contained for $type_name<$type_param> {
            fn container(&self) -> &ID {
                &self.container
            }

            fn set_container(&mut self, container: ID) {
                self.container = container
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations - ConnectionPointReference
// ------------------------------------------------------------------------------------------------

make_identified_impl!(ConnectionPointReference);

make_labeled_impl!(ConnectionPointReference);

make_contained_impl!(ConnectionPointReference);

impl ConnectionPointReference {
    pub fn within(container: ID) -> Self {
        Self {
            id: ID::random(),
            label: None,
            container,
            entry: vec![],
            exit: vec![],
            state: None,
        }
    }

    pub fn entry(&self) -> Iter<'_, ID> {
        self.entry.iter()
    }

    pub fn exit(&self) -> Iter<'_, ID> {
        self.exit.iter()
    }

    pub fn state(&self) -> &Option<ID> {
        &self.state
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations - PseudoStateKind
// ------------------------------------------------------------------------------------------------

impl Default for PseudoStateKind {
    fn default() -> Self {
        PseudoStateKind::Initial
    }
}

make_identified_impl!(PseudoState);

make_labeled_impl!(PseudoState);

make_contained_impl!(PseudoState);

impl PseudoState {
    pub fn within(container: ID, kind: PseudoStateKind) -> Self {
        Self {
            id: ID::random(),
            label: None,
            container,
            kind,
        }
    }

    pub fn kind(&self) -> PseudoStateKind {
        self.kind.clone()
    }

    pub fn is_initial(&self) -> bool {
        match self.kind {
            PseudoStateKind::Initial => true,
            _ => false,
        }
    }
    pub fn is_deep_history(&self) -> bool {
        match self.kind {
            PseudoStateKind::Initial => true,
            _ => false,
        }
    }
    pub fn is_shallow_history(&self) -> bool {
        match self.kind {
            PseudoStateKind::ShallowHistory => true,
            _ => false,
        }
    }
    pub fn is_join(&self) -> bool {
        match self.kind {
            PseudoStateKind::Join => true,
            _ => false,
        }
    }
    pub fn is_fork(&self) -> bool {
        match self.kind {
            PseudoStateKind::Fork => true,
            _ => false,
        }
    }
    pub fn is_junction(&self) -> bool {
        match self.kind {
            PseudoStateKind::Junction => true,
            _ => false,
        }
    }
    pub fn is_choice(&self) -> bool {
        match self.kind {
            PseudoStateKind::Choice => true,
            _ => false,
        }
    }
    pub fn is_entry_point(&self) -> bool {
        match self.kind {
            PseudoStateKind::EntryPoint => true,
            _ => false,
        }
    }
    pub fn is_exit_point(&self) -> bool {
        match self.kind {
            PseudoStateKind::ExitPoint => true,
            _ => false,
        }
    }
    pub fn is_terminate(&self) -> bool {
        match self.kind {
            PseudoStateKind::Terminate => true,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations - Region
// ------------------------------------------------------------------------------------------------

make_identified_impl!(Region, E);

make_labeled_impl!(Region, E);

make_contained_impl!(Region, E);

impl<E: 'static + PartialEq> Region<E> {
    pub fn within_state(container: ID) -> Self {
        Self {
            id: ID::random(),
            container,
            container_type: RegionContainerType::State,
            label: None,
            vertices: Rc::new(RefCell::new(vec![])),
            transitions: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn within_state_machine(container: ID) -> Self {
        Self {
            id: ID::random(),
            container,
            container_type: RegionContainerType::StateMachine,
            label: None,
            vertices: Rc::new(RefCell::new(vec![])),
            transitions: Rc::new(RefCell::new(vec![])),
        }
    }

    fn add_vertex(&self, vertex: Vertex<E>) {
        self.vertices.borrow_mut().push(Rc::new(vertex));
    }
    pub fn add_state(&self, state: State<E>) {
        self.add_vertex(Vertex::State(state));
    }

    pub fn new_simple_state(&self) -> ID {
        let new_state = State::within(self.id.clone());
        let new_id = new_state.id.clone();
        self.add_state(new_state);
        new_id
    }

    pub fn new_compound_state(&self) -> ID {
        let mut new_state = State::within(self.id.clone());
        let new_id = new_state.id.clone();
        let _ = new_state.new_region();
        self.add_state(new_state);
        new_id
    }

    pub fn new_orthogonal_state(&self) -> ID {
        let mut new_state = State::within(self.id.clone());
        let new_id = new_state.id.clone();
        let _ = new_state.new_region();
        let _ = new_state.new_region();
        self.add_state(new_state);
        new_id
    }

    pub fn new_final_state(&self) -> ID {
        let mut new_state = State::within(self.id.clone());
        let new_id = new_state.id.clone();
        new_state.final_state = true;
        self.add_state(new_state);
        new_id
    }

    pub fn new_initial_state(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::Initial)
    }

    pub fn new_deep_history_state(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::DeepHistory)
    }

    pub fn new_shallow_history_state(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::ShallowHistory)
    }

    pub fn new_join(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::Join)
    }

    pub fn new_fork(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::Fork)
    }

    pub fn new_junction(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::Junction)
    }

    pub fn new_choice_state(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::Choice)
    }

    pub fn new_entry_point(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::EntryPoint)
    }

    pub fn new_exit_point(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::ExitPoint)
    }

    pub fn new_terminate_state(&self) -> ID {
        self.new_pseudo_state(PseudoStateKind::Terminate)
    }

    fn new_pseudo_state(&self, kind: PseudoStateKind) -> ID {
        let new_state = PseudoState::within(self.id.clone(), kind);
        let new_id = new_state.id.clone();
        self.add_pseudo_state(new_state);
        new_id
    }

    pub fn add_pseudo_state(&self, pseudo_state: PseudoState) {
        self.add_vertex(Vertex::PseudoState(pseudo_state))
    }

    pub fn add_connection_point_ref(&self, cpr: ConnectionPointReference) {
        self.add_vertex(Vertex::ConnectionPointReference(cpr))
    }

    pub fn container_type(&self) -> &RegionContainerType {
        &self.container_type
    }

    pub fn vertices(&self) -> Vec<Rc<Vertex<E>>> {
        self.vertices.borrow().iter().cloned().collect()
    }

    pub fn transitions(&self) -> Vec<Rc<Transition<E>>> {
        self.transitions.borrow().iter().cloned().collect()
    }

    pub fn new_transition(&self, source: ID, target: ID) {
        let transition: Transition<E> = Transition::within(source, target, self.id.clone());
        self.add_transition(transition);
    }

    pub fn add_transition(&self, transition: Transition<E>) {
        self.transitions.borrow_mut().push(Rc::new(transition));
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations - State
// ------------------------------------------------------------------------------------------------

make_identified_impl!(State, E);

make_labeled_impl!(State, E);

make_contained_impl!(State, E);

make_has_regions_impl!(State);

impl<E: 'static + PartialEq> State<E> {
    pub fn within(container: ID) -> Self {
        Self {
            id: ID::random(),
            label: None,
            container,
            regions: vec![],
            sub_machine: None,
            connections: vec![],
            connection_points: vec![],
            deferrable_triggers: vec![],
            invariant: None,
            entry: None,
            do_activity: None,
            exit: None,
            final_state: false,
        }
    }

    pub fn new_region(&mut self) -> ID {
        let region: Region<E> = Region::within_state(self.id().clone());
        let region_id = region.id().clone();
        self.add_region(region);
        region_id
    }

    pub fn sub_machine(&self) -> &Option<ID> {
        &self.sub_machine
    }

    pub fn connections(&self) -> Iter<'_, ID> {
        self.connections.iter()
    }

    pub fn connection_points(&self) -> Iter<'_, ID> {
        self.connection_points.iter()
    }

    pub fn deferrable_triggers(&self) -> Iter<'_, Trigger<E>> {
        self.deferrable_triggers.iter()
    }

    pub fn invariant(&self) -> &Option<Box<dyn Constraint<E>>> {
        &self.invariant
    }

    pub fn entry(&self) -> &Option<Box<dyn Behavior<E>>> {
        &self.entry
    }

    pub fn do_activity(&self) -> &Option<Box<dyn Behavior<E>>> {
        &self.do_activity
    }

    pub fn exit(&self) -> &Option<Box<dyn Behavior<E>>> {
        &self.exit
    }

    pub fn is_composite(&self) -> bool {
        self.regions.len() == 1
    }

    pub fn is_orthogonal(&self) -> bool {
        self.regions.len() > 1
    }

    pub fn is_simple(&self) -> bool {
        self.regions.is_empty()
    }

    pub fn is_sub_machine_state(&self) -> bool {
        self.sub_machine.is_some()
    }

    pub fn is_final(&self) -> bool {
        self.final_state
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations - StateMachine
// ------------------------------------------------------------------------------------------------

impl<E: 'static + PartialEq> Default for StateMachine<E> {
    fn default() -> Self {
        let machine_id = ID::random();
        let mut new_machine = Self {
            id: machine_id.clone(),
            label: None,
            regions: vec![],
            sub_machine_states: vec![],
            connection_points: vec![],
            ref_machines: Default::default(),
            ref_vertices: Default::default(),
        };
        let _ = new_machine.new_region();
        new_machine
    }
}

make_identified_impl!(StateMachine, E);

make_labeled_impl!(StateMachine, E);

make_has_regions_impl!(StateMachine);

impl<E: PartialEq> Validate for StateMachine<E> {
    fn validate(&self) -> Result<()> {
        assert!(!self.regions.is_empty());
        Ok(())
    }
}

impl<E: 'static + PartialEq> StateMachine<E> {
    pub fn labeled(label: &str) -> Self {
        let mut machine: StateMachine<E> = StateMachine::default();
        machine.set_label(label);
        machine
    }

    pub fn new_region(&mut self) -> ID {
        let region: Region<E> = Region::within_state_machine(self.id().clone());
        let region_id = region.id().clone();
        self.add_region(region);
        region_id
    }

    pub fn has_sub_machine_states(&self) -> bool {
        !self.sub_machine_states.is_empty()
    }

    pub fn sub_machine_states(&self) -> Iter<'_, ID> {
        self.sub_machine_states.iter()
    }

    pub fn has_connection_points(&self) -> bool {
        !self.connection_points.is_empty()
    }

    pub fn connection_points(&self) -> Iter<'_, PseudoState> {
        self.connection_points.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations - StateMachineResolver
// ------------------------------------------------------------------------------------------------

impl<E: 'static + PartialEq> StateMachine<E> {
    pub fn find_machine(&self, machine: ID) -> Option<Rc<StateMachine<E>>> {
        self.ref_machines.borrow().get(&machine).cloned()
    }

    pub fn find_vertex(&self, container: ID, vertex: ID) -> Option<Rc<Vertex<E>>> {
        self.ref_vertices
            .borrow()
            .get(&(container, vertex))
            .cloned()
    }

    pub fn index_references(&self) {
        let regions = self.regions();
        for region in regions {
            self.add_reference_to_region(region);
        }
    }

    #[allow(dead_code)]
    fn add_reference_to_machine(&self, machine: Rc<StateMachine<E>>) {
        let _ = self
            .ref_machines
            .borrow_mut()
            .insert(machine.id.clone(), machine.clone());
        for region in &machine.regions {
            self.add_reference_to_region(region);
        }
    }

    fn add_reference_to_region(&self, region: &Region<E>) {
        for vertex in region.vertices() {
            self.add_reference_to_vertex(region.id(), vertex);
        }
    }

    fn add_reference_to_vertex(&self, container: &ID, vertex: Rc<Vertex<E>>) {
        let _ = self
            .ref_vertices
            .borrow_mut()
            .insert((container.clone(), vertex.id().clone()), vertex.clone());
        if vertex.is_state() {
            let state = vertex.as_state().unwrap();
            for region in &state.regions {
                self.add_reference_to_region(region);
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations - Transition
// ------------------------------------------------------------------------------------------------

make_labeled_impl!(Transition, E);

make_contained_impl!(Transition, E);

impl<E: PartialEq> Transition<E> {
    pub fn within(source: ID, target: ID, container: ID) -> Self {
        Self {
            label: None,
            container,
            kind: TransitionKind::Internal,
            source,
            target,
            triggers: vec![],
            guard: None,
            effect: None,
        }
    }

    pub fn source(&self) -> ID {
        self.source.clone()
    }

    pub fn target(&self) -> ID {
        self.target.clone()
    }

    pub fn has_triggers(&self) -> bool {
        !self.triggers.is_empty()
    }

    pub fn triggers(&self) -> Iter<'_, Trigger<E>> {
        self.triggers.iter()
    }

    pub fn has_guard(&self) -> bool {
        self.guard.is_some()
    }

    pub fn guard(&self) -> &Option<Box<dyn Constraint<E>>> {
        &self.guard
    }

    pub fn has_effect(&self) -> bool {
        self.effect.is_some()
    }

    pub fn effect(&self) -> &Option<Box<dyn Behavior<E>>> {
        &self.effect
    }

    pub fn kind(&self) -> TransitionKind {
        self.kind.clone()
    }

    pub fn is_internal(&self) -> bool {
        match self.kind {
            TransitionKind::Internal => true,
            _ => false,
        }
    }

    pub fn is_local(&self) -> bool {
        match self.kind {
            TransitionKind::Local => true,
            _ => false,
        }
    }

    pub fn is_external(&self) -> bool {
        match self.kind {
            TransitionKind::External => true,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations - Trigger
// ------------------------------------------------------------------------------------------------

impl<E: PartialEq> Default for Trigger<E> {
    fn default() -> Self {
        Self { event: None }
    }
}

impl<E: PartialEq> Trigger<E> {
    pub fn with_event(event: E) -> Self {
        Self { event: Some(event) }
    }

    pub fn event(&self) -> &Option<E> {
        &self.event
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations - Vertex
// ------------------------------------------------------------------------------------------------

impl<E: PartialEq> Vertex<E> {
    pub fn id(&self) -> &ID {
        match self {
            Vertex::State(state) => state.id(),
            Vertex::PseudoState(pseudo_state) => pseudo_state.id(),
            Vertex::ConnectionPointReference(cpr) => cpr.id(),
        }
    }

    pub fn is_state(&self) -> bool {
        match self {
            Vertex::State(_) => true,
            _ => false,
        }
    }

    pub fn as_state(&self) -> Option<&State<E>> {
        match self {
            Vertex::State(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn is_pseudo_state(&self) -> bool {
        match self {
            Vertex::PseudoState(_) => true,
            _ => false,
        }
    }

    pub fn as_pseudo_state(&self) -> Option<&PseudoState> {
        match self {
            Vertex::PseudoState(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn is_connection_point_reference(&self) -> bool {
        match self {
            Vertex::ConnectionPointReference(_) => true,
            _ => false,
        }
    }

    pub fn as_connection_point_reference(&self) -> Option<&ConnectionPointReference> {
        match self {
            Vertex::ConnectionPointReference(inner) => Some(inner),
            _ => None,
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

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::plant_uml::WritePlantUml;
    use crate::format::Stringify;

    #[allow(dead_code)]
    #[derive(PartialEq)]
    enum Event {
        This,
        That,
    }

    #[test]
    fn test_simple() {
        let simple: StateMachine<Event> = StateMachine::default();
        let region: &Region<Event> = simple.default_region().unwrap();
        let initial_id = region.new_initial_state();
        let state_id = region.new_simple_state();
        let final_id = region.new_final_state();

        region.new_transition(initial_id, state_id.clone());
        region.new_transition(state_id, final_id);

        assert!(simple.validate().is_ok());

        let writer = WritePlantUml::default();
        let string = writer.stringify(&simple);
        assert!(string.is_ok());
        println!("{}", string.unwrap());
    }
}
