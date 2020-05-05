/*!
Provides a visitor pattern for clients that want to review the model but do not always need the
details of ownership and hierarchy traversal.

More detailed description, with

# Example

*/

use crate::definition::id::ID;
use crate::definition::types::{
    Behavior, Constraint, HasRegions, Identified, Labeled, PseudoState, PseudoStateKind, Region,
    State, StateMachine, TransitionKind, Trigger, Validate, Vertex,
};
use crate::error::Error;
use std::borrow::Borrow;
use std::rc::Rc;
use std::slice::Iter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct Resolver<'a, E: PartialEq> {
    inner: &'a StateMachine<E>,
}

pub trait StateMachineVisitor<E: 'static + PartialEq> {
    #[allow(unused_variables)]
    fn enter_state_machine(
        &self,
        resolver: &Resolver<'_, E>,
        id: &ID,
        label: &Option<String>,
        machine_states: Iter<'_, ID>,
        connection_points: Iter<'_, PseudoState>,
    ) {
    }

    #[allow(unused_variables)]
    fn exit_state_machine(
        &self,
        resolver: &Resolver<'_, E>,
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
        resolver: &Resolver<'_, E>,
        id: &ID,
        label: &Option<String>,
        region_count: usize,
        sub_machine: &Option<ID>,
        connections: Iter<'_, ID>,
        connection_points: Iter<'_, ID>,
        deferrable_triggers: Iter<'_, Trigger<E>>,
        invariant: &Option<Box<dyn Constraint<E>>>,
        entry: &Option<Box<dyn Behavior<E>>>,
        do_activity: &Option<Box<dyn Behavior<E>>>,
        exit: &Option<Box<dyn Behavior<E>>>,
        is_final: bool,
    ) {
    }

    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    fn exit_state(
        &self,
        resolver: &Resolver<'_, E>,
        id: &ID,
        label: &Option<String>,
        region_count: usize,
        sub_machine: &Option<ID>,
        connections: Iter<'_, ID>,
        connection_points: Iter<'_, ID>,
        deferrable_triggers: Iter<'_, Trigger<E>>,
        invariant: &Option<Box<dyn Constraint<E>>>,
        entry: &Option<Box<dyn Behavior<E>>>,
        do_activity: &Option<Box<dyn Behavior<E>>>,
        exit: &Option<Box<dyn Behavior<E>>>,
        is_final: bool,
    ) {
    }

    #[allow(unused_variables)]
    fn enter_region(&self, resolver: &Resolver<'_, E>, id: &ID, label: &Option<String>) {}

    #[allow(unused_variables)]
    fn exit_region(&self, resolver: &Resolver<'_, E>, id: &ID, label: &Option<String>) {}

    #[allow(unused_variables)]
    fn connection_point_reference(
        &self,
        resolver: &Resolver<'_, E>,
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
        resolver: &Resolver<'_, E>,
        id: &ID,
        label: &Option<String>,
        kind: &PseudoStateKind,
    ) {
    }

    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    fn transition(
        &self,
        resolver: &Resolver<'_, E>,
        label: &Option<String>,
        kind: TransitionKind,
        source: ID,
        target: ID,
        triggers: Iter<'_, Trigger<E>>,
        guard: &Option<Box<dyn Constraint<E>>>,
        effect: &Option<Box<dyn Behavior<E>>>,
    ) {
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn visit_state_machine<E: 'static + PartialEq>(
    machine: &StateMachine<E>,
    visitor: &dyn StateMachineVisitor<E>,
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
    for region in machine.regions() {
        visit_region(region, &resolver, visitor)?;
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

impl<'a, E: 'static + PartialEq> Resolver<'a, E> {
    pub fn find_machine(&self, machine: ID) -> Option<Rc<StateMachine<E>>> {
        self.inner.find_machine(machine)
    }

    pub fn find_vertex(&self, container: ID, vertex: ID) -> Option<Rc<Vertex<E>>> {
        self.inner.find_vertex(container, vertex)
    }
}
// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn visit_state<E: 'static + PartialEq>(
    state: &State<E>,
    resolver: &Resolver<'_, E>,
    visitor: &dyn StateMachineVisitor<E>,
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
    for region in state.regions() {
        visit_region(region, resolver, visitor)?;
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

fn visit_region<E: 'static + PartialEq>(
    region: &Region<E>,
    resolver: &Resolver<'_, E>,
    visitor: &dyn StateMachineVisitor<E>,
) -> Result<(), Error> {
    visitor.enter_region(resolver, region.id(), region.label());
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
    visitor.exit_region(resolver, region.id(), region.label());
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
