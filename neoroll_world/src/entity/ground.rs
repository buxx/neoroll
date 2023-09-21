use super::Entity;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Ground {
    FreshWater,
    Soil,
}

impl Entity for Ground {}
