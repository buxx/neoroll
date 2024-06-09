use serde::{Deserialize, Serialize};

pub mod behavior;
pub mod build;
pub mod job;
pub mod need;
pub mod progress;
pub mod tribe;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum CollectType {
    Food,
}

/// All in game things have same unit
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Quantity(pub u64);
