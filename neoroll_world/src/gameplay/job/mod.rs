use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Job {
    Idle,
    SearchFood,
}

impl Default for Job {
    fn default() -> Self {
        Self::Idle
    }
}
