/*!
A Reasonably faithful implementation of the [Unified Modeling Language (UML)](http://uml.org/) State Machine.

The goal is to not just provide another state machine crate but to do so with a formal specification
that describes the description and execution semantics. This implementation is based upon the
[2.5.1](https://www.omg.org/spec/UML/2.5.1/PDF) version, dated 5th December 2017. The following
image is from §14.2.2 **Abstract Syntax** and provides a very useful overview of the components
of the model. As such, wherever possible the text of the documentation will reference the
specification, especially copies of the constraints and semantic rules.

![Abstract Syntax](https://raw.githubusercontent.com/johnstonskj/rust-uml_state_machine/master/doc/abstract-syntax.png)

The crate contains the descriptive model elements at the root level, with modules for error handling,
execution of instances and reading and writing formatted representations.

# Example

TBD

# Features

* `execution` - an in-memory execution environment for machines, included by default.
* `format-graphviz` - supports writing state diagrams with [GraphViz](https://graphviz.org/)
   ,following the style in [this post](https://martin-thoma.com/how-to-draw-a-finite-state-machine/).
* `format-plantuml` - supports writing [PlantUML](https://plantuml.com/state-diagram) state diagrams.
* `format-scxml` - supports writing [State Chart XML](https://www.w3.org/TR/scxml).
* `format-uml` - supports writing UML's [XML Metadata Interchange](https://www.omg.org/spec/XMI).
* `format-xstate` - supports writing JavaScript [state machines](https://xstate.js.org/).

# See Also

* [OMG Unified Modeling Language, Version 2.5.1](https://www.omg.org/spec/UML/2.5.1/PDF)
* [State Diagram (Wikipedia)](https://en.wikipedia.org/wiki/State_diagram)
* [UML State Machine (Wikipedia)](https://en.wikipedia.org/wiki/UML_state_machine)
* [StateMachines: A Visual Formalism for Complex Systems](https://www.inf.ed.ac.uk/teaching/courses/seoc/2005_2006/resources/StateMachines.pdf)
* [State Chart XML (SCXML): State Machine Notation for Control Abstraction](https://www.w3.org/TR/scxml/)
* [JavaScript state machines and StateMachines](https://xstate.js.org/)

*/

#![warn(
// ---------- Stylistic
future_incompatible,
nonstandard_style,
rust_2018_idioms,
trivial_casts,
trivial_numeric_casts,
// ---------- Public
// missing_debug_implementations,
// missing_docs,
unreachable_pub,
// ---------- Unsafe
unsafe_code,
// ---------- Unused
unused_extern_crates,
unused_import_braces,
unused_qualifications,
unused_results,
)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

///
/// The version of the UML specification from which this implementation is derived.
///
pub const UML_SPECIFICATION_VERSION: &str = "2.5.1";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod core;

pub mod error;

pub mod definition;

#[cfg(feature = "execution")]
pub mod execution;

pub mod format;
