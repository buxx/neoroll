use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct Progress(pub u8);

impl Progress {
    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    pub fn full(&self) -> bool {
        self.0 == 255
    }

    pub fn percent(&self) -> u8 {
        ((self.0 as f32 / 255.) * 100.) as u8
    }
}

impl From<f32> for Progress {
    fn from(value: f32) -> Self {
        Self((255. * value) as u8)
    }
}

impl From<Progress> for f32 {
    fn from(val: Progress) -> Self {
        val.0 as f32 / 255.
    }
}
