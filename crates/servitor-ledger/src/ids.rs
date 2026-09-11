use uuid::{Uuid, Version::SortRand};

use crate::ids::IdError::Uuidv7Required;

#[derive(Debug, thiserror::Error)]
pub enum IdError {
    #[error("UUID v7 Required")]
    Uuidv7Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TenantId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AccountId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct EntryId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutboxId(Uuid);

impl TenantId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Result<Self, IdError> {
        if id.get_version() != Some(SortRand) {
            Err(Uuidv7Required)
        } else {
            Ok(Self(id))
        }
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl AccountId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Result<Self, IdError> {
        if id.get_version() == Some(SortRand) {
            Ok(Self(id))
        } else {
            Err(Uuidv7Required)
        }
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl EntryId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Result<Self, IdError> {
        if id.get_version() == Some(SortRand) {
            Ok(Self(id))
        } else {
            Err(Uuidv7Required)
        }
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl LineId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Result<Self, IdError> {
        if id.get_version() == Some(SortRand) {
            Ok(Self(id))
        } else {
            Err(Uuidv7Required)
        }
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl OutboxId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Result<Self, IdError> {
        if id.get_version() == Some(SortRand) {
            Ok(Self(id))
        } else {
            Err(Uuidv7Required)
        }
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}
