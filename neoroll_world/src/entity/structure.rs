use super::Entity;

#[derive(Clone)]
pub enum Structure {
    Nothing,
    BigLeafTree,
}
impl Structure {
    pub fn hide(&self) -> bool {
        match self {
            Structure::Nothing => false,
            Structure::BigLeafTree => false,
        }
    }
}

impl Entity for Structure {}
