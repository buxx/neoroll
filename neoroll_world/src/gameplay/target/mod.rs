use serde::{Deserialize, Serialize};

use super::{material::Material, Quantity};

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    KeepStock(Material, TargetQuantity), // Quantity per habitant
}

/// All in game things have same unit
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum TargetQuantity {
    Fixed(Quantity),
    PerHuman(Quantity),
}
