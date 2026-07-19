//! Deliberately small types whose meaning is identical across bounded contexts.

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use thiserror::Error;
use uuid::Uuid;

macro_rules! identifier {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            #[must_use]
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Uuid::parse_str(value).map(Self)
            }
        }
    };
}

identifier!(WorkspaceId);
identifier!(PersonId);
identifier!(MemberId);
identifier!(DeviceId);
identifier!(CommandId);
identifier!(WorkspaceSessionId);
identifier!(JobId);
identifier!(OperationId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Revision(u64);

impl Revision {
    pub const INITIAL: Self = Self(1);

    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next monotonically increasing record revision.
    ///
    /// # Errors
    ///
    /// Returns [`RevisionError::Overflow`] when the current revision is
    /// `u64::MAX` and cannot be incremented without wrapping.
    pub fn next(self) -> Result<Self, RevisionError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(RevisionError::Overflow)
    }
}

impl Default for Revision {
    fn default() -> Self {
        Self::INITIAL
    }
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum RevisionError {
    #[error("record revision overflowed")]
    Overflow,
}

#[cfg(test)]
mod tests {
    use super::{PersonId, Revision, RevisionError};
    use std::str::FromStr;

    #[test]
    fn identifiers_round_trip_as_strings() {
        let id = PersonId::new();
        let parsed = PersonId::from_str(&id.to_string());
        assert_eq!(parsed, Ok(id));
    }

    #[test]
    fn revision_starts_at_one_and_increments() {
        let next = Revision::INITIAL.next();
        assert_eq!(next.map(Revision::get), Ok(2));
        assert_eq!(Revision::new(0), None);
    }

    #[test]
    fn revision_does_not_wrap_at_the_numeric_limit() {
        let maximum = Revision::new(u64::MAX);
        assert!(maximum.is_some());
        if let Some(maximum) = maximum {
            assert_eq!(maximum.next(), Err(RevisionError::Overflow));
        }
    }
}
