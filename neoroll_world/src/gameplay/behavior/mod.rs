use std::fmt::Display;

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

impl Display for Behavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Behavior::Idle => f.write_str("Idle"),
            Behavior::Collect(progress) => {
                f.write_str(&format!("Collect ({}%)", progress.percent()))
            }
            Behavior::DropOff => f.write_str("Drop off"),
            Behavior::MoveTo => f.write_str("Move"),
        }
    }
}
