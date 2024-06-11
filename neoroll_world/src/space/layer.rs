use crate::{
    entity::{floor::Floor, ground::Ground, structure::Structure},
    gameplay::{material::Material, Quantity},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Layers {
    grounds: FilledLayer<Ground>,
    floors: FilledLayer<Floor>,
    structures: CompositeLayer<Structure>,
    materials: FilledLayer<Vec<(Material, Quantity)>>,
}

impl Layers {
    pub fn new(
        grounds: FilledLayer<Ground>,
        floors: FilledLayer<Floor>,
        structures: CompositeLayer<Structure>,
        materials: FilledLayer<Vec<(Material, Quantity)>>,
    ) -> Self {
        Self {
            grounds,
            floors,
            structures,
            materials,
        }
    }

    pub fn grounds(&self) -> &FilledLayer<Ground> {
        &self.grounds
    }

    pub fn grounds_mut(&mut self) -> &mut FilledLayer<Ground> {
        &mut self.grounds
    }

    pub fn floors(&self) -> &FilledLayer<Floor> {
        &self.floors
    }

    pub fn floors_mut(&mut self) -> &mut FilledLayer<Floor> {
        &mut self.floors
    }

    pub fn structures(&self) -> &CompositeLayer<Structure> {
        &self.structures
    }

    pub fn structures_mut(&mut self) -> &mut CompositeLayer<Structure> {
        &mut self.structures
    }

    pub fn materials(&self) -> &FilledLayer<Vec<(Material, Quantity)>> {
        &self.materials
    }
}

impl Default for Layers {
    fn default() -> Self {
        Self {
            grounds: FilledLayer::new(vec![]),
            floors: FilledLayer::new(vec![]),
            structures: CompositeLayer::new(vec![]),
            materials: FilledLayer::new(vec![]),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FilledLayer<T> {
    items: Vec<T>,
}

impl<T> FilledLayer<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn get(&self, index: usize) -> &T {
        &self.items[index]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.items[index] = value;
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompositeLayer<T> {
    items: Vec<Option<T>>,
}

impl<T> CompositeLayer<T> {
    pub fn new(items: Vec<Option<T>>) -> Self {
        Self { items }
    }
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn get(&self, index: usize) -> &Option<T> {
        &self.items[index]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&mut self, index: usize, value: Option<T>) {
        self.items[index] = value;
    }

    pub fn items(&self) -> &[Option<T>] {
        &self.items
    }
}

impl<T> Default for CompositeLayer<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}
