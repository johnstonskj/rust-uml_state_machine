/*!
One-line description.

More detailed description, with

# Example

*/

use crate::core::{Context, ID};
use crate::definition::types::StateMachine;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct StateMachineInstance {
    id: ID,
    chart: Rc<StateMachine>,
    active: HashSet<ID>,
    context: RefCell<Context>,
    state: RefCell<ExecutionState>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
