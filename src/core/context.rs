/*!
One-line description.

More detailed description, with

# Example

*/

use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Bool(bool),
    Byte(u8),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Rc<RefCell<Array>>),
    Object(Rc<RefCell<Object>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    inner: RefCell<Vec<FieldValue>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    inner: RefCell<HashMap<FieldName, FieldValue>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldName(String);

#[derive(Clone, Debug, PartialEq)]
pub struct FieldPath(Vec<FieldName>);

#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    root: FieldValue,
}

pub trait Compound<K> {
    fn contains_key(&self, key: K) -> bool;
    fn get(&self, key: K) -> Option<FieldValue>;
    fn insert(&self, key: K, value: FieldValue);
    fn remove(&self, key: K) -> Option<FieldValue>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Array {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl From<Vec<FieldValue>> for Array {
    fn from(value: Vec<FieldValue>) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
}

impl From<FieldValue> for Array {
    fn from(value: FieldValue) -> Self {
        Self {
            inner: RefCell::new(vec![value]),
        }
    }
}

impl Compound<usize> for Array {
    fn contains_key(&self, key: usize) -> bool {
        key < self.inner.borrow().len()
    }

    fn get(&self, key: usize) -> Option<FieldValue> {
        self.inner.borrow().get(key).cloned()
    }

    fn insert(&self, key: usize, value: FieldValue) {
        self.inner.borrow_mut().insert(key, value)
    }

    fn remove(&self, key: usize) -> Option<FieldValue> {
        if key < self.inner.borrow().len() {
            None
        } else {
            Some(self.inner.borrow_mut().remove(key))
        }
    }

    fn len(&self) -> usize {
        self.inner.borrow().len()
    }
}

impl Array {
    pub fn push(&self, value: FieldValue) {
        self.inner.borrow_mut().push(value)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Object {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl From<HashMap<FieldName, FieldValue>> for Object {
    fn from(value: HashMap<FieldName, FieldValue, RandomState>) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl Compound<FieldName> for Object {
    fn contains_key(&self, key: FieldName) -> bool {
        self.inner.borrow().contains_key(&key)
    }

    fn get(&self, key: FieldName) -> Option<FieldValue> {
        self.inner.borrow().get(&key).cloned()
    }

    fn insert(&self, key: FieldName, value: FieldValue) {
        let _ = self.inner.borrow_mut().insert(key, value);
    }

    fn remove(&self, key: FieldName) -> Option<FieldValue> {
        self.inner.borrow_mut().remove(&key)
    }

    fn len(&self) -> usize {
        self.inner.borrow().len()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for FieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for FieldName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty()
            && s.chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            Ok(Self(s.to_string()))
        } else {
            Err(())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for FieldPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("/")
        )
    }
}

impl FromStr for FieldPath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mapped: Result<Vec<FieldName>, _> =
            s.split('/').map(|s| FieldName::from_str(s)).collect();
        match mapped {
            Ok(mapped) => Ok(Self(mapped)),
            Err(_) => Err(()),
        }
    }
}

impl FieldPath {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<&FieldName> {
        self.0.first()
    }

    pub fn rest(&self) -> FieldPath {
        let names = self.0.iter().skip(1).cloned().collect();
        FieldPath(names)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<bool> for FieldValue {
    fn from(value: bool) -> Self {
        FieldValue::Bool(value)
    }
}

impl From<u8> for FieldValue {
    fn from(value: u8) -> Self {
        FieldValue::Byte(value)
    }
}

impl From<i64> for FieldValue {
    fn from(value: i64) -> Self {
        FieldValue::Integer(value)
    }
}

impl From<f64> for FieldValue {
    fn from(value: f64) -> Self {
        FieldValue::Float(value)
    }
}

impl From<String> for FieldValue {
    fn from(value: String) -> Self {
        FieldValue::String(value)
    }
}

impl From<Array> for FieldValue {
    fn from(value: Array) -> Self {
        FieldValue::Array(Rc::new(RefCell::new(value)))
    }
}

impl From<Object> for FieldValue {
    fn from(value: Object) -> Self {
        FieldValue::Object(Rc::new(RefCell::new(value)))
    }
}

impl FieldValue {
    pub fn is_simple(&self) -> bool {
        match self {
            FieldValue::Bool(_)
            | FieldValue::Byte(_)
            | FieldValue::Integer(_)
            | FieldValue::Float(_)
            | FieldValue::String(_) => true,
            _ => false,
        }
    }

    pub fn is_compound(&self) -> bool {
        match self {
            FieldValue::Array(_) | FieldValue::Object(_) => true,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Context {
    fn default() -> Self {
        Self {
            root: FieldValue::Object(Default::default()),
        }
    }
}

impl From<Object> for Context {
    fn from(value: Object) -> Self {
        Self {
            root: FieldValue::Object(Rc::new(RefCell::new(value))),
        }
    }
}

impl Compound<FieldPath> for Context {
    fn contains_key(&self, key: FieldPath) -> bool {
        match self.find(&key) {
            None => false,
            Some((container, key)) => match container {
                FieldValue::Array(array) => match usize::from_str(&key.to_string()) {
                    Ok(key) => array.borrow().contains_key(key),
                    Err(_) => false,
                },
                FieldValue::Object(object) => object.borrow().contains_key(key),
                _ => false,
            },
        }
    }

    fn get(&self, key: FieldPath) -> Option<FieldValue> {
        match self.find(&key) {
            None => None,
            Some((container, key)) => match container {
                FieldValue::Array(array) => match usize::from_str(&key.to_string()) {
                    Ok(key) => array.borrow().get(key),
                    Err(_) => None,
                },
                FieldValue::Object(object) => object.borrow().get(key),
                _ => None,
            },
        }
    }

    fn insert(&self, key: FieldPath, value: FieldValue) {
        match self.find(&key) {
            None => (),
            Some((container, key)) => match container {
                FieldValue::Array(array) => {
                    if let Ok(key) = usize::from_str(&key.to_string()) {
                        array.borrow_mut().insert(key, value);
                    }
                }
                FieldValue::Object(object) => object.borrow_mut().insert(key, value),
                _ => (),
            },
        }
    }

    fn remove(&self, key: FieldPath) -> Option<FieldValue> {
        match self.find(&key) {
            None => None,
            Some((container, key)) => match container {
                FieldValue::Array(array) => match usize::from_str(&key.to_string()) {
                    Ok(key) => array.borrow_mut().remove(key),
                    Err(_) => None,
                },
                FieldValue::Object(object) => object.borrow_mut().remove(key),
                _ => None,
            },
        }
    }

    fn len(&self) -> usize {
        match &self.root {
            FieldValue::Object(object) => object.borrow().len(),
            _ => 0,
        }
    }
}

impl Context {
    fn find(&self, key: &FieldPath) -> Option<(FieldValue, FieldName)> {
        self.find_in(key, &self.root)
    }

    fn find_in(&self, key: &FieldPath, container: &FieldValue) -> Option<(FieldValue, FieldName)> {
        if key.is_empty() {
            None
        } else if key.len() == 1 {
            let name = key.first().unwrap();
            Some((container.clone(), name.clone()))
        } else {
            let name = key.first().unwrap();
            match container {
                FieldValue::Array(array) => match usize::from_str(&name.to_string()) {
                    Ok(key) => array.borrow().get(key),
                    Err(_) => None,
                },
                FieldValue::Object(object) => object.borrow().get(name.clone()),
                _ => None,
            }
            .and_then(|v| {
                if v.is_compound() {
                    self.find_in(&key.rest(), &v)
                } else {
                    None
                }
            })
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
