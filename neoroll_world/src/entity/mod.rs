use serde::{Deserialize, Serialize};

pub mod creature;
pub mod floor;
pub mod ground;
pub mod structure;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
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

impl From<f32> for Filled {
    fn from(value: f32) -> Self {
        Self((255. * value) as u8)
    }
}

impl From<Filled> for f32 {
    fn from(val: Filled) -> Self {
        val.0 as f32 / 255.
    }
}
