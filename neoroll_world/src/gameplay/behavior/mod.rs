use serde::{Deserialize, Serialize};

use super::progress::Progress;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Behavior {
    Idle,
    Collect(Progress),
    DropOff,
    MoveTo,
}
impl Behavior {
    pub fn progress(&self) -> Option<&Progress> {
        match self {
            Behavior::Idle => None,
            Behavior::Collect(progress) => Some(progress),
            Behavior::DropOff => None,
            Behavior::MoveTo => None,
        }
    }
}

impl Default for Behavior {
    fn default() -> Self {
        Self::Idle
    }
}
