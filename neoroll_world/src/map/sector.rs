use super::element::Element;

#[derive(Debug, Clone)]
pub struct SectorRelativePoint(pub f32, pub f32);

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
