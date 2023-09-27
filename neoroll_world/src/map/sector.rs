use super::element::Element;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SectorRelativePoint(pub f32, pub f32);

impl From<Vec2> for SectorRelativePoint {
    fn from(value: Vec2) -> Self {
        Self(value.x, value.y)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sector {
    elements: Vec<(SectorRelativePoint, Element)>,
}

impl Sector {
    pub fn new(elements: Vec<(SectorRelativePoint, Element)>) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &[(SectorRelativePoint, Element)] {
        self.elements.as_ref()
    }
}
