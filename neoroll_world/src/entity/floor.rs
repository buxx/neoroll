use super::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Floor {
    Nothing,
    ShortGrass,
}
impl Floor {
    pub fn hide(&self) -> bool {
        match self {
            Floor::Nothing => false,
            Floor::ShortGrass => true,
        }
    }
}

impl Entity for Floor {}
