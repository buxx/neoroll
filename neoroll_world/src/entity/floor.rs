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
}

impl Entity for Floor {}
