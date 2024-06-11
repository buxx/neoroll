use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub enum Ground {
    FreshWater,
    Soil,
}
