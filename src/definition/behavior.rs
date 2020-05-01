/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::StateID;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub type ConditionFn<E, D> = Rc<dyn Fn(&StateID, &Option<E>, &D) -> bool>;

pub struct Condition<E, D> {
    label: Option<String>,
    condition: ConditionFn<E, D>,
}

pub type ActionFn<D> = Rc<dyn Fn(&StateID, &D)>;

pub type MutActionFn<D> = Rc<dyn Fn(&StateID, &mut D)>;

enum ActionChoice<D> {
    Immutable(ActionFn<D>),
    Mutable(MutActionFn<D>),
}

pub struct Action<D> {
    label: Option<String>,
    action: ActionChoice<D>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<E, D> Debug for Condition<E, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Condition")
            .field("label", &self.label)
            .field("condition", &String::from("..."))
            .finish()
    }
}

impl<E, D> Condition<E, D> {
    pub fn new(condition: ConditionFn<E, D>) -> Self {
        Self {
            label: None,
            condition,
        }
    }

    pub fn with_label(condition: ConditionFn<E, D>, label: &str) -> Self {
        Self {
            label: Some(label.to_string()),
            condition,
        }
    }

    pub fn evaluate(&self, in_state: &StateID, on_event: &Option<E>, context: &D) -> bool {
        let condition = &self.condition;
        condition(in_state, on_event, context)
    }

    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }
}

// ------------------------------------------------------------------------------------------------

impl<D> Debug for ActionChoice<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            ActionChoice::Immutable(_) => "Immutable",
            ActionChoice::Mutable(_) => "Mutable",
        };
        f.debug_struct("ActionChoice")
            .field(kind, &String::from(".."))
            .finish()
    }
}

// ------------------------------------------------------------------------------------------------

impl<D> Debug for Action<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Action")
            .field("label", &self.label)
            .field("action", &self.action)
            .finish()
    }
}

impl<D> Action<D> {
    pub fn new(action: ActionFn<D>) -> Self {
        Self {
            label: None,
            action: ActionChoice::Immutable(action),
        }
    }

    pub fn with_label(action: ActionFn<D>, label: &str) -> Self {
        Self {
            label: Some(label.to_string()),
            action: ActionChoice::Immutable(action),
        }
    }

    pub fn call(&self, in_state: &StateID, context: &mut D) {
        match &self.action {
            ActionChoice::Immutable(action) => action(in_state, context),
            ActionChoice::Mutable(action) => action(in_state, context),
        }
    }

    pub fn label(&self) -> Option<String> {
        self.label.clone()
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
