use std::{fmt::Display, marker::PhantomData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord,Serialize, Deserialize,Copy
)]
pub struct TypedId<T> {
    value: Uuid,
    _phantom: PhantomData<T>,
}

impl<T> Display for TypedId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> TypedId<T> {
    /// Create a new typed ID from a string value
    pub fn from(value: Uuid) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    /// Generate a new unique ID
    pub fn generate() -> Self {
        Self::from(Uuid::now_v7())
    }

    /// Get the value of the ID
    pub fn value(&self) -> &Uuid {
        &self.value
    }

    /// Consume and return the inner string
    pub fn into_inner(self) -> Uuid {
        self.value
    }
}
