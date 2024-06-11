use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};
use worldgen::constraint;
use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{NoiseMap, NoiseMapGenerator, Seed, Size, Step};
use worldgen::world::tile::{Constraint, ConstraintType};
use worldgen::world::{Tile, World};

use crate::entity::floor::Floor;
use crate::entity::ground::Ground;
use crate::entity::structure::Structure;
use crate::entity::Filled;
use crate::space::layer::{CompositeLayer, FilledLayer, Layers};
use crate::space::world::World as GeneratedWorld;

use super::WorldGenerator;

#[derive(Clone)]
pub enum TileLike {
    Water,
    Plain,
    Forest,
}

pub struct PerlinNoiseSimpleGenerator {
    seed: String,
    lines: i64,
    columns: i64,
    nm1_from: f64,
    nm1_to: f64,
    nm2_from: f64,
    nm2_to: f64,
    nm2_factor: i64,
}

#[allow(clippy::too_many_arguments)]
impl PerlinNoiseSimpleGenerator {
    pub fn new(
        seed: &str,
        lines: i64,
        columns: i64,
        nm1_from: f64,
        nm1_to: f64,
        nm2_from: f64,
        nm2_to: f64,
        nm2_factor: i64,
    ) -> Self {
        Self {
            seed: seed.to_string(),
            lines,
            columns,
            nm1_from,
            nm1_to,
            nm2_from,
            nm2_to,
            nm2_factor,
        }
    }
}

impl WorldGenerator for PerlinNoiseSimpleGenerator {
    fn generate(&self) -> GeneratedWorld {
        let noise = PerlinNoise::new();

        let nm1 = NoiseMap::new(noise)
            .set(Seed::of(&self.seed))
            .set(Step::of(self.nm1_from, self.nm1_to));

        let nm2 = NoiseMap::new(noise)
            .set(Seed::of(&self.seed))
            .set(Step::of(self.nm2_from, self.nm2_to));
        let nm = Box::new(nm1 + nm2 * self.nm2_factor);

        let world = World::new()
            .set(Size::of(self.columns, self.lines))
            .add(Tile::new(TileLike::Water).when(constraint!(nm.clone(), < -0.1)))
            .add(Tile::new(TileLike::Plain).when(constraint!(nm.clone(), < 0.10)))
            .add(Tile::new(TileLike::Forest));

        let mut grounds = vec![];
        let mut floors = vec![];
        let mut structures = vec![];
        let mut materials = vec![];

        for row in world.generate(0, 0).iter() {
            for val in row.iter() {
                for tile in val.iter() {
                    let (ground, floor, structure) = match tile {
                        TileLike::Water => (Ground::FreshWater, Floor::Nothing, None),
                        TileLike::Plain => {
                            let floor = [
                                Floor::ShortGrass,
                                Floor::FruitBush(Filled::full()),
                                Floor::Nothing,
                            ][WalkerTableBuilder::new(&[60, 20, 20]).build().next()]
                            .clone();
                            let structure = [Some(Structure::BigLeafTree), None]
                                [WalkerTableBuilder::new(&[10, 90]).build().next()]
                            .clone();
                            (Ground::Soil, floor, structure)
                        }
                        TileLike::Forest => {
                            let structure = [
                                Some(Structure::BigLeafTree),
                                Some(Structure::FruitTree(Filled::full())),
                                None,
                            ][WalkerTableBuilder::new(&[60, 20, 20]).build().next()]
                            .clone();
                            (Ground::Soil, Floor::Nothing, structure)
                        }
                    };

                    grounds.push(ground);
                    floors.push(floor);
                    structures.push(structure);
                    materials.push(vec![]);
                }
            }
        }

        GeneratedWorld::new(
            Layers::new(
                FilledLayer::new(grounds),
                FilledLayer::new(floors),
                CompositeLayer::new(structures),
                FilledLayer::new(materials),
            ),
            self.lines as usize,
            self.columns as usize,
            vec![],
        )
    }
}
