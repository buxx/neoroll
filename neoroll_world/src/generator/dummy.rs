use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::{
    entity::{floor::Floor, ground::Ground, structure::Structure, Filled},
    space::{
        layer::{CompositeLayer, FilledLayer, Layers},
        world::World,
    },
};

use super::WorldGenerator;

#[derive(Default)]
pub struct DummyWorldGenerator {
    lines: usize,
    columns: usize,
}

impl DummyWorldGenerator {
    pub fn new(lines: usize, columns: usize) -> Self {
        Self { lines, columns }
    }

    fn floor(&self) -> Floor {
        let choices = [Floor::ShortGrass, Floor::Nothing];
        let index_weights = [80, 20];
        choices[WalkerTableBuilder::new(&index_weights).build().next()].clone()
    }

    fn structure(&self) -> Option<Structure> {
        let choices = [Some(Structure::BigLeafTree(Filled::full())), None];
        let index_weights = [80, 20];
        choices[WalkerTableBuilder::new(&index_weights).build().next()].clone()
    }
}

impl WorldGenerator for DummyWorldGenerator {
    fn generate(&self) -> World {
        let mut grounds = vec![];
        let mut floors = vec![];
        let mut structures = vec![];
        let mut materials = vec![];

        for _ in 0..self.lines {
            for _ in 0..self.columns {
                grounds.push(Ground::Soil);
                floors.push(self.floor());
                structures.push(self.structure());
                materials.push(vec![]);
            }
        }

        World::new(
            Layers::new(
                FilledLayer::new(grounds),
                FilledLayer::new(floors),
                CompositeLayer::new(structures),
                FilledLayer::new(materials),
            ),
            self.lines,
            self.columns,
            vec![],
        )
    }
}
