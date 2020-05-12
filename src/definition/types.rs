/*!
Types and implementations for constructing a state machine model.

More detailed description, with

# Example

*/

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::slice::Iter;

use crate::core::ID;
use crate::error::Result;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

pub trait Event: Debug {}

pub trait HasRegions: Identified {
    fn has_regions(&self) -> bool;

    fn regions(&self) -> Iter<'_, Region>;

    fn region(&self, index: usize) -> Option<&Region>;

    fn default_region(&self) -> Option<&Region> {
        self.region(0)
    }

    fn add_region(&mut self, region: Region);
}

pub trait Labeled {
    fn label(&self) -> &Option<String>;

    fn set_label(&mut self, label: &str);

    fn unset_label(&mut self);
}

pub trait Identified {
    fn id(&self) -> &ID;
}

pub trait Contained {
    fn container(&self) -> &ID;

    fn set_container(&mut self, container: ID);
}

pub trait Validate {
    fn validate(&self) -> Result<()>;
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The top-level state chart type itself.
///
pub struct StateMachine {
    pub(crate) id: ID,
    pub(crate) label: Option<String>,
    /// **UML**: `{subsets ownedMember} +region 1..*`
    pub(crate) regions: Vec<Region>,
    /// **UML**: `+submachineState * : State`
    pub(crate) sub_machine_states: Vec<ID>,
    /// **UML**: `{subsets ownedMember} +connectionPoint *`
    pub(crate) connection_points: Vec<PseudoState>,
    pub(crate) ref_machines: RefCell<HashMap<ID, Rc<StateMachine>>>,
    pub(crate) ref_vertices: RefCell<HashMap<(ID, ID), Rc<Vertex>>>,
}

// ------------------------------------------------------------------------------------------------

pub enum RegionContainerType {
    State,
    StateMachine,
}

pub enum Vertex {
    State(State),
    PseudoState(PseudoState),
    ConnectionPointReference(ConnectionPointReference),
}

pub struct Region {
    pub(crate) id: ID,
    pub(crate) label: Option<String>,
    /// **UML**: `{subsets namespace} +stateMachine 0..1 : StateMachine`
    pub(crate) container: ID,
    pub(crate) container_type: RegionContainerType,
    /// **UML**: `{subsets ownedMember} +subvertex *`
    pub(crate) vertices: Rc<RefCell<Vec<Rc<Vertex>>>>,
    /// **UML**: `{subsets ownedMember} +transition *`
    pub(crate) transitions: Rc<RefCell<Vec<Rc<Transition>>>>,
}

// ------------------------------------------------------------------------------------------------

pub struct Trigger {
    pub(crate) event: Option<Box<dyn Event>>,
}

// ------------------------------------------------------------------------------------------------

pub struct State {
    pub(crate) id: ID,
    pub(crate) label: Option<String>,
    /// **UML**: `{subsets namespace} +container 0..1 : Region`
    pub(crate) container: ID,
    /// **UML**: `{subsets ownedMember} +region *`
    pub(crate) regions: Vec<Region>,
    /// **UML**: `+ submachine 0..1 : StateMachine`
    pub(crate) sub_machine: Option<ID>,
    /// **UML**: `{subsets ownedMember} +connection * : ConnectionPointReference`
    pub(crate) connections: Vec<ID>,
    /// **UML**: `{subsets ownedMember} +connectionPoint * : PseudoState`
    pub(crate) connection_points: Vec<ID>,
    /// **UML**: `{subsets ownedElement} +deferrableTrigger *`
    pub(crate) deferrable_triggers: Vec<Trigger>,
    /// **UML**: `{subsets ownedRule} +stateInvariant 0..1`
    pub(crate) invariant: Option<Box<dyn Constraint>>,
    /// **UML**: `{subsets ownedElement} +entry 0..1`
    pub(crate) entry: Option<Box<dyn Behavior>>,
    /// **UML**: `{subsets ownedElement} +doActivity 0..1`
    pub(crate) do_activity: Option<Box<dyn Behavior>>,
    /// **UML**: `{subsets ownedElement} +exit 0..1`
    pub(crate) exit: Option<Box<dyn Behavior>>,
    pub(crate) final_state: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PseudoStateKind {
    Initial,
    DeepHistory,
    ShallowHistory,
    Join,
    Fork,
    Junction,
    Choice,
    EntryPoint,
    ExitPoint,
    Terminate,
}

pub struct PseudoState {
    pub(crate) id: ID,
    pub(crate) label: Option<String>,
    /// **UML**: `{subsets namespace} +container 0..1 : Region`
    pub(crate) container: ID,
    /// default = Initial
    pub(crate) kind: PseudoStateKind,
}

// ------------------------------------------------------------------------------------------------

pub struct ConnectionPointReference {
    pub(crate) id: ID,
    pub(crate) label: Option<String>,
    /// **UML**: `{subsets namespace} +container 0..1 : Region`
    pub(crate) container: ID,
    /// **UML**: `+entry * : PseudoState`
    pub(crate) entry: Vec<ID>,
    /// **UML**: `+exit * : PseudoState`
    pub(crate) exit: Vec<ID>,
    /// **UML**: `{subsets namespace} +state 0..1 : State`
    pub(crate) state: Option<ID>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum TransitionKind {
    Internal,
    Local,
    External,
}

pub struct Transition {
    pub(crate) label: Option<String>,
    /// **UML**: `{subsets namespace} + container 1 : Region`
    pub(crate) container: ID,
    /// **UML**: `default = External`
    pub(crate) kind: TransitionKind,
    /// **UML**: `+source 1 : Vertex`
    pub(crate) source: ID,
    /// **UML**: `+target 1 : Vertex`
    pub(crate) target: ID,
    /// **UML**: `{subsets ownedElement} +trigger *`
    pub(crate) triggers: Vec<Trigger>,
    /// **UML**: `{subsets ownedRule} +guard 0..1`
    pub(crate) guard: Option<Box<dyn Constraint>>,
    /// **UML**: `{subsets ownedElement} +effect 0..1`
    pub(crate) effect: Option<Box<dyn Behavior>>,
}

// ------------------------------------------------------------------------------------------------

pub trait Behavior: Labeled {
    fn perform(&self, in_state: &ID, on_trigger: &Trigger);
}

pub trait Constraint: Labeled {
    fn evaluate(&self, in_state: &ID, on_trigger: &Trigger) -> bool;
}
