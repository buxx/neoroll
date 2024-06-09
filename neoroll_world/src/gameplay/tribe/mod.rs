pub mod settings;
pub mod structure;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Tribe {
    id: TribeId,
}

impl Tribe {
    pub fn new(id: TribeId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &TribeId {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TribeId(Uuid);

impl TribeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TribeId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for TribeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
