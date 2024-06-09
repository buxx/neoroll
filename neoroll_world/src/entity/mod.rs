use serde::{Deserialize, Serialize};

pub mod creature;
pub mod floor;
pub mod ground;
pub mod structure;

pub trait Entity {}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Filled(pub u8);

impl Filled {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn full() -> Self {
        Self(255)
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
