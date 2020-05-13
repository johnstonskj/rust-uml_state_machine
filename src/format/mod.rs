/*!
Provides support for parsing, and emitting, external representations of a `StateMachine`.

Each sub-module provides for a different representation type but may not implement both the
`Parse` and `Stringify` traits.

# Example

```rust
use uml_state_machine::definition::types::*;
use uml_state_machine::format::plant_uml::WritePlantUml;
use uml_state_machine::format::Stringify;

let simple: StateMachine = StateMachine::default();
let region: &Region = simple.default_region().unwrap();
let initial_id = region.new_initial_state();
let state_id = region.new_simple_state();
let final_id = region.new_final_state();

region.new_transition(initial_id, state_id.clone());
region.new_transition(state_id, final_id);

let writer = WritePlantUml::default();
let string = writer.stringify(&simple);
```

*/

use crate::definition::types::StateMachine;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Parse an external representation and return a new `StateMachine` model.
///
pub trait Parse {
    type Error;

    fn parse(&self, string: &str) -> Result<StateMachine, Self::Error>;
}

///
/// Create a textual representation of the state machine.
///
pub trait Stringify {
    type Error;

    fn stringify(&self, machine: &StateMachine) -> Result<String, Self::Error>;
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "format-graphviz")]
pub mod graphviz;

#[cfg(feature = "format-plantuml")]
pub mod plant_uml;

#[cfg(feature = "format-scxml")]
pub mod scxml;

#[cfg(feature = "format-uml")]
pub mod uml;

#[cfg(feature = "format-xstate")]
pub mod xstate;
