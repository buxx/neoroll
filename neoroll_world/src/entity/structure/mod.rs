use crate::gameplay::build::Buildable;

use super::{Entity, Filled};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Structure {
    Nothing,
    BigLeafTree,
    FruitTree(Filled),
    Campfire,
}

impl Structure {
    pub fn hide(&self) -> bool {
        match self {
            Structure::Nothing => false,
            Structure::BigLeafTree => false,
            Structure::FruitTree(_) => false,
            Structure::Campfire => false,
        }
    }
}

impl Entity for Structure {}

impl From<Buildable> for Structure {
    fn from(value: Buildable) -> Self {
        match value {
            Buildable::Campfire => Structure::Campfire,
        }
    }
}
