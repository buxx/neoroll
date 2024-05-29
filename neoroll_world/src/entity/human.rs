use super::Entity;
use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Human {
    position: Vec3,
}
impl Human {
    pub fn new(&self, position: Vec3) -> Self {
        Self { position }
    }
}

impl Entity for Human {}
