use super::Entity;

#[derive(Clone)]
pub enum Floor {
    Nothing,
    ShortGrass,
}
impl Floor {
    pub fn hide(&self) -> bool {
        match self {
            Floor::Nothing => false,
            Floor::ShortGrass => true,
        }
    }
}

impl Entity for Floor {}
