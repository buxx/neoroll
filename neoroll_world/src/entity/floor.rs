use crate::gameplay::{CollectType, Quantity};

use super::{Entity, Filled};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Floor {
    Nothing,
    ShortGrass,
    FruitBush(Filled),
}
impl Floor {
    pub fn hide(&self) -> bool {
        match self {
            Floor::Nothing => false,
            Floor::ShortGrass => true,
            Floor::FruitBush(_) => true,
        }
    }

    pub fn collect_base_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Floor::Nothing => todo!(),
                Floor::ShortGrass => todo!(),
                Floor::FruitBush(_) => todo!(),
            },
        }
    }

    pub fn collectable(&self, type_: CollectType) -> Option<&Filled> {
        match type_ {
            CollectType::Food => match self {
                Floor::Nothing | Floor::ShortGrass => None,
                Floor::FruitBush(filled) => Some(filled),
            },
        }
    }
}

impl Entity for Floor {}
