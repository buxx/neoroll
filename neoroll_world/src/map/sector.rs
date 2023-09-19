use glam::Vec2;

use super::element::Element;

#[derive(Debug, Clone)]
pub struct SectorRelativePoint(pub f32, pub f32);

impl From<Vec2> for SectorRelativePoint {
    fn from(value: Vec2) -> Self {
        Self(value.x, value.y)
    }
}

#[derive(Debug, Clone)]
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
