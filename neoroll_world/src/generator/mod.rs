use crate::space::world::EntireWorld;

pub mod dummy;

pub trait WorldGenerator {
    fn generate(&self) -> EntireWorld;
}
