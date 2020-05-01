/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StateID(String);

pub mod error {
    error_chain! {
        errors {
            #[doc = "`StateID` may not be an empty string."]
            EmptyString {
                description("`StateID` may not be an empty string.")
                display("`StateID` may not be an empty string.")
            }
            #[doc = "`StateID` contains invalid character(s)."]
            InvalidCharacter {
                description("`StateID` contains invalid character(s).")
                display("`StateID` contains invalid character(s).")
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for StateID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StateID {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(s.to_string()))
    }
}

const INVALID_STATE_TAG_VALUE: &str = "<invalid-state-tag>";
const TAG_SEPARATOR: &str = "::";

impl StateID {
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

    pub fn append_random(&self) -> error::Result<Self> {
        Ok(Self(format!(
            "{}{}{}",
            self.0,
            TAG_SEPARATOR,
            Self::random()
        )))
    }

    pub fn split(&self) -> Vec<StateID> {
        self.0
            .split(TAG_SEPARATOR)
            .filter_map(|s| {
                if StateID::validate(s).is_ok() {
                    Some(StateID::from_str(s).unwrap())
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
