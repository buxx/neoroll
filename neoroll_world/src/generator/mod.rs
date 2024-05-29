use crate::space::world::World;

pub mod dummy;
pub mod perlin_noise_simple;

pub trait WorldGenerator {
    fn generate(&self) -> World;
}
