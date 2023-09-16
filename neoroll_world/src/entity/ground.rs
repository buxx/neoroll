use super::Entity;

#[derive(Clone)]
pub enum Ground {
    Soil,
}

impl Entity for Ground {}
