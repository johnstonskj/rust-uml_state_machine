/*!
Writes a state machine out in the [PlantUML](https://plantuml.com/) format. Does not support
parsing PlantUML.

# Example

```rust
use uml_state_machine::definition::types::StateMachine;
use uml_state_machine::format::plant_uml::WritePlantUml;
use uml_state_machine::format::Stringify;

pub fn print_machine(machine: &StateMachine) -> Option<String> {
    let writer = WritePlantUml::default();
    writer.stringify(&machine).ok()
}
```

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use std::cell::RefCell;
use std::marker::PhantomData;
use std::slice::Iter;

use crate::core::ID;
use crate::definition::types::Identified;
use crate::definition::types::{
    Behavior, Constraint, PseudoState, PseudoStateKind, StateMachine, TransitionKind, Trigger,
    Vertex,
};
use crate::definition::visitor::{visit_state_machine, Resolver, StateMachineVisitor};
use crate::format::Stringify;
use std::borrow::Borrow;

///
/// Implements Stringify for PlantUML.
///
pub struct WritePlantUml {
    ph: PhantomData<u8>,
}

struct Visitor {
    container: RefCell<Vec<ID>>,
    buffer: RefCell<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for WritePlantUml {
    fn default() -> Self {
        Self { ph: PhantomData }
    }
}

impl Stringify for WritePlantUml {
    type Error = ();

    fn stringify(&self, machine: &StateMachine) -> Result<String, Self::Error> {
        let visitor = Visitor {
            container: Default::default(),
            buffer: RefCell::new(String::new()),
        };
        visitor.push_line("@startuml");
        let _ = visit_state_machine(&machine, &visitor);
        visitor.push_line("@enduml");
        Ok(visitor.buffer.into_inner())
    }
}

impl StateMachineVisitor for Visitor {
    fn enter_state_machine(
        &self,
        _: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        _: Iter<'_, ID>,
        _: Iter<'_, PseudoState>,
    ) {
        self.container.borrow_mut().push(id.clone());
        if let Some(label) = label {
            self.push_str("title ");
            self.push_line(label);
        }
    }

    fn exit_state_machine(
        &self,
        _: &Resolver<'_>,
        _: &ID,
        _: &Option<String>,
        _: Iter<'_, ID>,
        _: Iter<'_, PseudoState>,
    ) {
        let _ = self.container.borrow_mut().pop();
    }

    fn enter_state(
        &self,
        _resolver: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        region_count: usize,
        _sub_machine: &Option<ID>,
        _connections: Iter<'_, ID>,
        _connection_points: Iter<'_, ID>,
        _deferrable_triggers: Iter<'_, Trigger>,
        _invariant: &Option<Box<dyn Constraint>>,
        _entry: &Option<Box<dyn Behavior>>,
        _do_activity: &Option<Box<dyn Behavior>>,
        _exit: &Option<Box<dyn Behavior>>,
        is_final: bool,
    ) {
        self.container.borrow_mut().push(id.clone());
        if !is_final {
            if let Some(label) = label {
                self.push_str(&format!("state \"{}\" as {}", label, id));
            } else {
                self.push_str(&format!("state {}", id));
            }
            if region_count > 0 {
                self.push_str(" {");
            }
            self.push_line("");
        }
    }

    fn exit_state(
        &self,
        _resolver: &Resolver<'_>,
        id: &ID,
        _label: &Option<String>,
        region_count: usize,
        _sub_machine: &Option<ID>,
        _connections: Iter<'_, ID>,
        _connection_points: Iter<'_, ID>,
        _deferrable_triggers: Iter<'_, Trigger>,
        _invariant: &Option<Box<dyn Constraint>>,
        entry: &Option<Box<dyn Behavior>>,
        do_activity: &Option<Box<dyn Behavior>>,
        exit: &Option<Box<dyn Behavior>>,
        is_final: bool,
    ) {
        if !is_final {
            if region_count > 0 {
                self.push_line("}");
            }
            if let Some(entry) = entry {
                if let Some(label) = entry.label() {
                    self.push_line(&format!("{}: entry / {}", id, label));
                } else {
                    self.push_line(&format!("{}: entry / ()", id));
                }
            }
            if let Some(do_activity) = do_activity {
                if let Some(label) = do_activity.label() {
                    self.push_line(&format!("{}: do / {}", id, label));
                } else {
                    self.push_line(&format!("{}: do / ()", id));
                }
            }
            if let Some(exit) = exit {
                if let Some(label) = exit.label() {
                    self.push_line(&format!("{}: exit / {}", id, label));
                } else {
                    self.push_line(&format!("{}: exit / ()", id));
                }
            }
        }
        let _ = self.container.borrow_mut().pop();
    }

    fn enter_region(
        &self,
        _resolver: &Resolver<'_>,
        id: &ID,
        _label: &Option<String>,
        _last: bool,
    ) {
        self.container.borrow_mut().push(id.clone());
    }

    fn exit_region(&self, _resolver: &Resolver<'_>, _: &ID, _label: &Option<String>, last: bool) {
        if !last {
            self.push_line("--");
        }
        let _ = self.container.borrow_mut().pop();
    }

    fn pseudo_state(
        &self,
        _resolver: &Resolver<'_>,
        id: &ID,
        label: &Option<String>,
        kind: &PseudoStateKind,
    ) {
        fn pseudo_as_state(visitor: &Visitor, id: &ID, label: &Option<String>, stereotype: &str) {
            if let Some(label) = label {
                visitor.push_str(&format!("state \"{}\" as {} <<{}>>", label, id, stereotype));
            } else {
                visitor.push_str(&format!("state {} <<{}>>", id, stereotype));
            }
        }
        match kind {
            PseudoStateKind::Initial => {}
            PseudoStateKind::Choice => pseudo_as_state(self, id, label, "choice"),
            PseudoStateKind::Fork => pseudo_as_state(self, id, label, "fork"),
            PseudoStateKind::Join => pseudo_as_state(self, id, label, "join"),
            _ => unimplemented!(),
        }
    }

    fn transition(
        &self,
        resolver: &Resolver<'_>,
        label: &Option<String>,
        _kind: TransitionKind,
        source: ID,
        target: ID,
        _triggers: Iter<'_, Trigger>,
        guard: &Option<Box<dyn Constraint>>,
        effect: &Option<Box<dyn Behavior>>,
    ) {
        fn state_str(resolver: &Resolver<'_>, container: ID, id: ID) -> String {
            match resolver.find_vertex(container.clone(), id) {
                None => "ERROR".to_string(),
                Some(rc_vertex) => match rc_vertex.borrow() {
                    Vertex::State(state) => {
                        if state.is_final() {
                            "[*]".to_string()
                        } else {
                            state.id().to_string()
                        }
                    }
                    Vertex::PseudoState(pseudo_state) => {
                        if pseudo_state.is_initial() {
                            "[*]".to_string()
                        } else {
                            pseudo_state.id().to_string()
                        }
                    }
                    Vertex::ConnectionPointReference(_) => "CPR".to_string(),
                },
            }
        }
        let container = self.container.borrow().last().unwrap().clone();
        self.push_str(&format!(
            "{} --> {}",
            state_str(resolver, container.clone(), source),
            state_str(resolver, container.clone(), target)
        ));
        let mut all_label = String::new();
        if let Some(guard) = guard {
            if let Some(label) = guard.label() {
                all_label.push_str(&format!("[{}] ", label));
            }
        }
        if let Some(label) = label {
            all_label.push_str(&format!("{} ", label));
        }
        if let Some(effect) = effect {
            if let Some(label) = effect.label() {
                all_label.push_str(&format!("/ {} ", label));
            }
        }
        if !all_label.is_empty() {
            self.push_line(&format!(" : {}", all_label));
        } else {
            self.push_line("");
        }
    }
}

impl Visitor {
    pub(crate) fn push_str(&self, string: &str) {
        self.buffer.borrow_mut().push_str(string);
    }

    pub(crate) fn push_line(&self, string: &str) {
        self.buffer.borrow_mut().push_str(&format!("{}\n", string));
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
