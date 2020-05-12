/*!
A common identifier type, it provides for string-based random values but also allows for prefixes
and ID paths.

# Example

```rust
use uml_state_machine::core::ID;

let first_id = ID::random_with_prefix("thing").unwrap();
let _next_id = first_id.append_random();
```
*/

use std::fmt::Display;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The ID type, this is purposefully opaque, but can be represented as a `String` and parsed from a
/// `String`.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ID(String);

///
/// Provides a common error implementation, error kind enumeration, and constrained result type for
/// ID creation/parsing.
///
pub mod error {
    error_chain! {
        errors {
            #[doc = "`ID` may not be an empty string."]
            EmptyString {
                description("`ID` may not be an empty string.")
                display("`ID` may not be an empty string.")
            }
            #[doc = "`ID` contains invalid character(s)."]
            InvalidCharacter {
                description("`ID` contains invalid character(s).")
                display("`ID` contains invalid character(s).")
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn default_split_separator() -> String {
    TAG_SEPARATOR.to_string()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

trait IDValueGenerator: Sync {
    fn next(&self) -> String;
    fn invalid_value(&self) -> String;
    fn is_valid_value(&self, s: &str) -> bool {
        self.is_valid_prefix(s)
    }
    fn is_valid_prefix(&self, s: &str) -> bool {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':')
    }
}

lazy_static! {
    static ref IDGENERATOR: Box<dyn IDValueGenerator> =
        Box::new(generator::IntegerGenerator::default());
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ID {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(error::ErrorKind::EmptyString.into())
        } else if IDGENERATOR.is_valid_value(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(error::ErrorKind::InvalidCharacter.into())
        }
    }
}

const TAG_SEPARATOR: &str = "::";

impl ID {
    pub fn random() -> Self {
        Self(IDGENERATOR.next())
    }

    pub fn random_with_prefix(prefix: &str) -> error::Result<Self> {
        if prefix.is_empty() {
            Err(error::ErrorKind::EmptyString.into())
        } else if IDGENERATOR.is_valid_prefix(prefix) {
            Ok(Self(format!(
                "{}{}{}",
                prefix,
                TAG_SEPARATOR,
                Self::random()
            )))
        } else {
            Err(error::ErrorKind::InvalidCharacter.into())
        }
    }

    pub fn invalid() -> Self {
        Self(IDGENERATOR.invalid_value())
    }

    pub fn append_random(&self) -> Self {
        Self(format!("{}{}{}", self.0, TAG_SEPARATOR, Self::random()))
    }

    pub fn split(&self) -> Vec<ID> {
        self.0
            .split(TAG_SEPARATOR)
            .filter_map(|s| {
                if IDGENERATOR.is_valid_value(s) {
                    Some(ID::from_str(s).unwrap())
                } else {
                    None
                }
            })
            .collect()
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

mod generator {
    use super::IDValueGenerator;
    use std::cell::RefCell;
    use std::ops::Add;

    #[derive(Debug)]
    pub(super) struct StringGenerator {}

    impl Default for StringGenerator {
        fn default() -> Self {
            Self {}
        }
    }

    #[allow(unsafe_code)]
    unsafe impl Sync for StringGenerator {}

    impl IDValueGenerator for StringGenerator {
        fn next(&self) -> String {
            blob_uuid::random_blob()
        }

        fn invalid_value(&self) -> String {
            "<invalid-state-tag>".to_string()
        }
    }

    // --------------------------------------------------------------------------------------------

    #[derive(Debug)]
    pub(super) struct IntegerGenerator {
        current: RefCell<i64>,
    }

    impl Default for IntegerGenerator {
        fn default() -> Self {
            Self {
                current: RefCell::new(0),
            }
        }
    }

    #[allow(unsafe_code)]
    unsafe impl Sync for IntegerGenerator {}

    impl IDValueGenerator for IntegerGenerator {
        fn next(&self) -> String {
            let value = *self.current.borrow();
            *self.current.borrow_mut() = value + 1;
            value.to_string()
        }

        fn invalid_value(&self) -> String {
            i64::MIN.to_string()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let first_id = ID::random_with_prefix("thing").unwrap();
        let _next_id = first_id.append_random();
    }
}
