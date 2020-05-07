/*!
Provides a visitor pattern for clients that want to review the model but do not always need the
details of ownership and hierarchy traversal.



# Example

*/

use std::borrow::Borrow;
use std::rc::Rc;
use std::slice::Iter;

use crate::core::ID;
use crate::definition::types::{
    Behavior, Constraint, HasRegions, Identified, Labeled, PseudoState, PseudoStateKind, Region,
    State, StateMachine, TransitionKind, Trigger, Validate, Vertex,
};
use crate::error::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct Resolver<'a> {
    inner: &'a StateMachine,
}

pub trait StateMachineVisitor {
    #[allow(unused_variables)]
    fn enter_state_machine(
        &self,
        resolver: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        machine_states: Iter<'_, ID>,
        connection_points: Iter<'_, PseudoState>,
    ) {
    }

    #[allow(unused_variables)]
    fn exit_state_machine(
        &self,
        resolver: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        machine_states: Iter<'_, ID>,
        connection_points: Iter<'_, PseudoState>,
    ) {
    }

    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    fn enter_state(
        &self,
        resolver: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        region_count: usize,
        sub_machine: &Option<ID>,
        connections: Iter<'_, ID>,
        connection_points: Iter<'_, ID>,
        deferrable_triggers: Iter<'_, Trigger>,
        invariant: &Option<Box<dyn Constraint>>,
        entry: &Option<Box<dyn Behavior>>,
        do_activity: &Option<Box<dyn Behavior>>,
        exit: &Option<Box<dyn Behavior>>,
        is_final: bool,
    ) {
    }

    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    fn exit_state(
        &self,
        resolver: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        region_count: usize,
        sub_machine: &Option<ID>,
        connections: Iter<'_, ID>,
        connection_points: Iter<'_, ID>,
        deferrable_triggers: Iter<'_, Trigger>,
        invariant: &Option<Box<dyn Constraint>>,
        entry: &Option<Box<dyn Behavior>>,
        do_activity: &Option<Box<dyn Behavior>>,
        exit: &Option<Box<dyn Behavior>>,
        is_final: bool,
    ) {
    }

    #[allow(unused_variables)]
    fn enter_region(&self, resolver: &Resolver<'_>, id: &ID, label: &Option<String>, last: bool) {}

    #[allow(unused_variables)]
    fn exit_region(&self, resolver: &Resolver<'_>, id: &ID, label: &Option<String>, last: bool) {}

    #[allow(unused_variables)]
    fn connection_point_reference(
        &self,
        resolver: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        entry: Iter<'_, ID>,
        exit: Iter<'_, ID>,
        state: &Option<ID>,
    ) {
    }

    #[allow(unused_variables)]
    fn pseudo_state(
        &self,
        resolver: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        kind: &PseudoStateKind,
    ) {
    }

    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    fn transition(
        &self,
        resolver: &Resolver<'_>,
        label: &Option<String>,
        kind: TransitionKind,
        source: ID,
        target: ID,
        triggers: Iter<'_, Trigger>,
        guard: &Option<Box<dyn Constraint>>,
        effect: &Option<Box<dyn Behavior>>,
    ) {
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn visit_state_machine(
    machine: &StateMachine,
    visitor: &dyn StateMachineVisitor,
) -> Result<(), Error> {
    machine.validate()?;
    machine.index_references();
    let resolver = Resolver { inner: machine };
    visitor.enter_state_machine(
        &resolver,
        machine.id(),
        machine.label(),
        machine.sub_machine_states(),
        machine.connection_points(),
    );
    let regions = machine.regions();
    let num_regions = regions.len();
    for (index, region) in regions.enumerate() {
        visit_region(region, &resolver, visitor, index == num_regions - 1)?;
    }
    visitor.exit_state_machine(
        &resolver,
        machine.id(),
        machine.label(),
        machine.sub_machine_states(),
        machine.connection_points(),
    );
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Resolver<'a> {
    pub fn find_machine(&self, machine: ID) -> Option<Rc<StateMachine>> {
        self.inner.find_machine(machine)
    }

    pub fn find_vertex(&self, container: ID, vertex: ID) -> Option<Rc<Vertex>> {
        self.inner.find_vertex(container, vertex)
    }
}
// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn visit_state(
    state: &State,
    resolver: &Resolver<'_>,
    visitor: &dyn StateMachineVisitor,
) -> Result<(), Error> {
    visitor.enter_state(
        resolver,
        state.id(),
        state.label(),
        state.regions.len(),
        state.sub_machine(),
        state.connections(),
        state.connection_points(),
        state.deferrable_triggers(),
        state.invariant(),
        state.entry(),
        state.do_activity(),
        state.exit(),
        state.is_final(),
    );
    let regions = state.regions();
    let num_regions = regions.len();
    for (index, region) in regions.enumerate() {
        visit_region(region, resolver, visitor, index == num_regions - 1)?;
    }
    visitor.exit_state(
        resolver,
        state.id(),
        state.label(),
        state.regions.len(),
        state.sub_machine(),
        state.connections(),
        state.connection_points(),
        state.deferrable_triggers(),
        state.invariant(),
        state.entry(),
        state.do_activity(),
        state.exit(),
        state.is_final(),
    );
    Ok(())
}

fn visit_region(
    region: &Region,
    resolver: &Resolver<'_>,
    visitor: &dyn StateMachineVisitor,
    last: bool,
) -> Result<(), Error> {
    visitor.enter_region(resolver, region.id(), region.label(), last);
    for vertex in region.vertices() {
        match vertex.borrow() {
            Vertex::State(state) => {
                visit_state(state, resolver, visitor)?;
            }
            Vertex::PseudoState(pseudo_state) => {
                visitor.pseudo_state(
                    resolver,
                    pseudo_state.id(),
                    pseudo_state.label(),
                    &pseudo_state.kind(),
                );
            }
            Vertex::ConnectionPointReference(cpr) => {
                visitor.connection_point_reference(
                    resolver,
                    cpr.id(),
                    cpr.label(),
                    cpr.entry(),
                    cpr.exit(),
                    cpr.state(),
                );
            }
        }
    }
    for transition in region.transitions() {
        visitor.transition(
            resolver,
            transition.label(),
            transition.kind(),
            transition.source(),
            transition.target(),
            transition.triggers(),
            transition.guard(),
            transition.effect(),
        );
    }
    visitor.exit_region(resolver, region.id(), region.label(), last);
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
