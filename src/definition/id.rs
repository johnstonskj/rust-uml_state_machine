/*!
A common identifier type, it provides for string-based random values but also allows for prefixes
and ID paths.

# Example

```rust
use uml_state_machine::id::ID;

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

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ID {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(s.to_string()))
    }
}

const INVALID_STATE_TAG_VALUE: &str = "<invalid-state-tag>";
const TAG_SEPARATOR: &str = "::";

impl ID {
    pub fn random() -> Self {
        Self(blob_uuid::random_blob())
    }

    pub fn random_with_prefix(prefix: &str) -> error::Result<Self> {
        Self::validate(prefix)?;
        Ok(Self(format!(
            "{}{}{}",
            prefix,
            TAG_SEPARATOR,
            Self::random()
        )))
    }

    pub fn invalid() -> Self {
        Self(INVALID_STATE_TAG_VALUE.to_string())
    }

    pub fn append(&self, suffix: &str) -> error::Result<Self> {
        Self::validate(suffix)?;
        Ok(Self(format!("{}{}{}", self.0, TAG_SEPARATOR, suffix)))
    }

    pub fn append_random(&self) -> Self {
        Self(format!("{}{}{}", self.0, TAG_SEPARATOR, Self::random()))
    }

    pub fn split(&self) -> Vec<ID> {
        self.0
            .split(TAG_SEPARATOR)
            .filter_map(|s| {
                if ID::validate(s).is_ok() {
                    Some(ID::from_str(s).unwrap())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn is_valid(&self) -> bool {
        Self::validate(&self.0).is_ok()
    }

    fn validate(s: &str) -> error::Result<()> {
        if s.is_empty() {
            Err(error::ErrorKind::EmptyString.into())
        } else if !s
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':')
        {
            Err(error::ErrorKind::InvalidCharacter.into())
        } else {
            Ok(())
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
