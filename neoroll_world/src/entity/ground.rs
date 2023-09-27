use super::Entity;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub enum Ground {
    FreshWater,
    Soil,
}

impl Entity for Ground {}
