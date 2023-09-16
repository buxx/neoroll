use crate::entity::{floor::Floor, ground::Ground, structure::Structure, Entity};

pub struct Layers {
    grounds: FilledLayer<Ground>,
    floors: FilledLayer<Floor>,
    structures: CompositeLayer<Structure>,
}

impl Layers {
    pub fn new(
        grounds: FilledLayer<Ground>,
        floors: FilledLayer<Floor>,
        structures: CompositeLayer<Structure>,
    ) -> Self {
        Self {
            grounds,
            floors,
            structures,
        }
    }

    pub fn grounds(&self) -> &FilledLayer<Ground> {
        &self.grounds
    }

    pub fn floors(&self) -> &FilledLayer<Floor> {
        &self.floors
    }

    pub fn structures(&self) -> &CompositeLayer<Structure> {
        &self.structures
    }
}

pub struct FilledLayer<T: Entity> {
    items: Vec<T>,
}

impl<T: Entity> FilledLayer<T> {
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
}

pub struct CompositeLayer<T: Entity> {
    items: Vec<Option<T>>,
}

impl<T: Entity> CompositeLayer<T> {
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
}
