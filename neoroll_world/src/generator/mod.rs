use crate::space::world::EntireWorld;

pub mod dummy;
pub mod perlin_noise_simple;

pub trait WorldGenerator {
    fn generate(&self) -> EntireWorld;
}
