use crate::gameplay::{build::Buildable, CollectType, Quantity};

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

    pub fn filled(&self) -> Option<&Filled> {
        match self {
            Structure::Nothing | Structure::BigLeafTree | Structure::Campfire => todo!(),
            Structure::FruitTree(filled) => Some(filled),
        }
    }

    pub fn collect_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Structure::Nothing | Structure::BigLeafTree | Structure::Campfire => None,
                Structure::FruitTree(_) => Some(Quantity(1000)),
            },
        }
    }

    pub fn maximum_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Structure::Nothing | Structure::BigLeafTree | Structure::Campfire => None,
                Structure::FruitTree(_) => Some(Quantity(25000)),
            },
        }
    }

    pub fn collectable(&self, type_: CollectType) -> Option<&Filled> {
        match type_ {
            CollectType::Food => match self {
                Structure::Nothing | Structure::BigLeafTree => None,
                Structure::FruitTree(filled) => Some(filled),
                Structure::Campfire => None,
            },
        }
    }

    // TODO: is that a good arch ?
    pub fn with_filled(&self, filled: Filled) -> Structure {
        match self {
            Structure::Nothing | Structure::BigLeafTree | Structure::Campfire => self.clone(),
            Structure::FruitTree(_) => Structure::FruitTree(filled),
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
