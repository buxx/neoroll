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

// TODO: trait for collect/reduced/etc
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
            Structure::Nothing | Structure::BigLeafTree | Structure::Campfire => None,
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

    pub fn with_filled(&self, filled: Filled) -> Structure {
        match self {
            Structure::Nothing | Structure::BigLeafTree | Structure::Campfire => self.clone(),
            Structure::FruitTree(_) => Structure::FruitTree(filled),
        }
    }

    pub fn reduced(&self, type_: CollectType) -> Structure {
        match self {
            Structure::Nothing | Structure::BigLeafTree | Structure::Campfire => self.clone(),
            Structure::FruitTree(filled) => {
                let maximum_quantity = self
                    .maximum_quantity(type_)
                    .expect("Structure with reduce must own a maximum quantity");
                let collect_quantity = self
                    .collect_quantity(type_)
                    .expect("Structure with reduce must own a collect quantity");
                let current_quantity: u64 =
                    (maximum_quantity.0 as f32 * (filled.0 as f32 / 255.)) as u64;
                let new_quantity_ = current_quantity - collect_quantity.0.min(current_quantity);
                let new_filled_ = ((new_quantity_ as f32 / maximum_quantity.0 as f32) * 255.) as u8;

                let new_filled = Filled::new(new_filled_);

                self.with_filled(new_filled)
            }
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
