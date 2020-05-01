/*!
Another state chart crate.

An attempt to make a faithful state chart implementation with compound, parallel state and history.

# Example

TBD

# See

* [State Diagram (Wikipedia)](https://en.wikipedia.org/wiki/State_diagram)
* [UML State Machine (Wikipedia)](https://en.wikipedia.org/wiki/UML_state_machine)
* [StateMachines: A Visual Formalism for Complex Systems](https://www.inf.ed.ac.uk/teaching/courses/seoc/2005_2006/resources/StateMachines.pdf)
* [State Chart XML (SCXML): State Machine Notation for Control Abstraction](https://www.w3.org/TR/scxml/)
* [JavaScript state machines and StateMachines](https://xstate.js.org/)

*/

#![warn(
    missing_debug_implementations,
//    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error {
    error_chain! {
        errors {
            #[doc = "`StateMachine::states` may not be empty."]
            ChartStatesEmpty {
                description("`StateMachine::states` may not be empty.")
                display("`StateMachine::states` may not be empty.")
            }

            #[doc = "`StateMachine::initial` is set to the ID of a non-initial state."]
            ChartInvalidInitialStateKind {
                description("`StateMachine::initial` is set to the ID of a non-initial state.")
                display("`StateMachine::initial` is set to the ID of a non-initial state.")
            }

            #[doc = "`StateMachine::initial` is set to the ID of a non-existent state."]
            ChartInvalidInitialStateName {
                description("`StateMachine::initial` is set to the ID of a non-existent state.")
                display("`StateMachine::initial` is set to the ID of a non-existent state.")
            }

            #[doc = "`StateMachine::states` contains no final states."]
            ChartNoFinalState {
                description("`StateMachine::states` contains no final states.")
                display("`StateMachine::states` contains no final states.")
            }

            #[doc = "`State::child_states` may not be empty for `StateKind::Compound` or `StateKind::Parallel`."]
            StateChildStatesEmpty {
                description("`State::child_states` may not be empty for `StateKind::Compound` or `StateKind::Parallel`.")
                display("`State::child_states` may not be empty for `StateKind::Compound` or `StateKind::Parallel`.")
            }

            #[doc = "`State::initial` is either missing or not a valid initial state."]
            StateInitialState {
                description("`State::initial` is either missing or not a valid initial state.")
                display("`State::initial` is either missing or not a valid initial state.")
            }

            #[doc = "`StateKind::Initial` states may not have inbound transitions."]
            InitialStateTransitions {
                description("`StateKind::Initial` states may not have inbound transitions.")
                display("`StateKind::Initial` states may not have inbound transitions.")
            }

            #[doc = "`StateKind::Final` states may not have outbound transitions."]
            FinalStateTransitions {
                description("`StateKind::Final` states may not have outbound transitions.")
                display("`StateKind::Final` states may not have outbound transitions.")
            }

            #[doc = "`Transition` must have at least one of `event`, `target`, or `conditions`."]
            TransitionTrigger {
                description("Transition must have at least one of `event`, `target`, or `conditions`.")
                display("Transition must have at least one of `event`, `target`, or `conditions`.")
            }

            #[doc = "`Transition::target` is either missing or not a valid initial state."]
            TransitionTargetState {
                description("`Transition::target` is either missing or not a valid initial state.")
                display("`Transition::target` is either missing or not a valid initial state`.")
            }

            #[doc = "`State` has multiple live outbound transitions."]
            StateMultipleOutbound {
                description("State has multiple live outbound transitions.")
                display("State has multiple live outbound transitions.")
            }

            #[doc = "`StateMachineInstance` is already in a done state."]
            InstanceIsDone {
                description("`StateMachineInstance` is already in a done state.")
                display("`StateMachineInstance` is already in a done state.")
            }

            #[doc = "`StateMachineInstance::is_active` is true, `execute` may only be called once."]
            InstanceIsActive {
                description("`StateMachineInstance::is_active` is true, `execute` may only be called once.")
                display("`StateMachineInstance::is_active` is true, `execute` may only be called once.")
            }

            #[doc = "`StateMachineInstance::is_active` is false, `execute` must be called before `post`."]
            InstanceIsNotActive {
                description("`StateMachineInstance::is_active` is false, `execute` must be called before `post`.")
                display("`StateMachineInstance::is_active` is false, `execute` must be called before `post`.")
            }

            #[doc = "More than one transition is active for an active state."]
            MoreThanOneTransition {
                description("More than one transition is active for an active state.")
                display("More than one transition is active for an active state.")
            }

            #[doc = "An action executed for an active state panicked."]
            ActionPanicked {
                description("An action executed for an active state panicked.")
                display("An action executed for an active state panicked.")
            }

            #[doc = "An event may not be posted while an action is running in a synchronous execution."]
            EventDuringAction {
                description("An event may not be posted while an action is running in a synchronous execution.")
                display("An event may not be posted while an action is running in a synchronous execution.")
            }
        }
    }
}

pub use error::{Error, ErrorKind, Result};

pub mod definition;
pub use definition::{ActionFn, ConditionFn, State, StateKind, StateMachine, Transition};

pub mod execution;
pub use execution::StateMachineInstance;

pub mod format;

pub mod tag;
pub use tag::StateID;
