use std::collections::HashMap;

use crate::{
    entity::{
        creature::{Creature, CreatureChange, CreatureId},
        floor::Floor,
        ground::Ground,
        structure::Structure,
    },
    gameplay::tribe::{structure::StructureOwn, TribeId},
    space::{layer::Layers, AbsoluteWorldPoint},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct World {
    layers: Layers,
    lines: usize,
    columns: usize,
    creatures: HashMap<CreatureId, Creature>,
    tribes_creatures: HashMap<TribeId, Vec<CreatureId>>,
}

impl World {
    pub fn new(layers: Layers, lines: usize, columns: usize, creatures: Vec<Creature>) -> Self {
        let mut tribes_creatures = HashMap::new();
        for creature in &creatures {
            tribes_creatures
                .entry(*creature.tribe_id())
                .or_insert(vec![])
                .push(*creature.id());
        }

        let creatures = creatures
            .into_iter()
            .map(|c| (*c.id(), c))
            .collect::<HashMap<CreatureId, Creature>>();

        Self {
            layers,
            lines,
            columns,
            creatures,
            tribes_creatures,
        }
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn ground(&self, point: &AbsoluteWorldPoint) -> Option<&Ground> {
        // Outside
        if point.row_i().0 >= self.lines as isize
            || point.row_i().0 < 0
            || point.col_i().0 >= self.columns as isize
            || point.col_i().0 < 0
        {
            return None;
        }

        let row_i = point.row_i().0 as usize;
        let col_i = point.col_i().0 as usize;

        let i = row_i * self.columns + col_i;
        Some(self.layers.grounds().get(i))
    }

    pub fn floor(&self, point: &AbsoluteWorldPoint) -> Option<&Floor> {
        // Outside
        if point.row_i().0 >= self.lines as isize
            || point.row_i().0 < 0
            || point.col_i().0 >= self.columns as isize
            || point.col_i().0 < 0
        {
            return None;
        }

        let row_i = point.row_i().0 as usize;
        let col_i = point.col_i().0 as usize;

        let i = row_i * self.columns + col_i;
        Some(self.layers.floors().get(i))
    }

    pub fn structure(&self, point: &AbsoluteWorldPoint) -> &Option<Structure> {
        // Outside
        if point.row_i().0 >= self.lines as isize
            || point.row_i().0 < 0
            || point.col_i().0 >= self.columns as isize
            || point.col_i().0 < 0
        {
            return &None;
        }

        let row_i = point.row_i().0 as usize;
        let col_i = point.col_i().0 as usize;

        let i = row_i * self.columns + col_i;
        self.layers.structures().get(i)
    }

    pub fn creatures(&self) -> &HashMap<CreatureId, Creature> {
        &self.creatures
    }

    pub fn add_creature(&mut self, creature: Creature) {
        self.tribes_creatures
            .entry(*creature.tribe_id())
            .or_default()
            .push(*creature.id());
        self.creatures.insert(*creature.id(), creature);
    }

    // FIXME: don't permit modify self.creatures by outside to be able to ensure tribes_creatures integrity
    pub fn creatures_mut(&mut self) -> &mut HashMap<CreatureId, Creature> {
        &mut self.creatures
    }

    pub fn set_structure(&mut self, point: AbsoluteWorldPoint, structure: Option<Structure>) {
        let row_i = point.row_i().0 as usize;
        let col_i = point.col_i().0 as usize;

        let i = row_i * self.columns + col_i;
        self.layers.structures_mut().set(i, structure);
    }

    pub fn contains(&self, point: &AbsoluteWorldPoint) -> bool {
        point.0 .0 >= 0
            && point.1 .0 >= 0
            && point.0 .0 < self.lines as isize
            && point.1 .0 < self.columns as isize
    }

    pub fn tribe_creature_ids(&self, tribe_id: &TribeId) -> Option<&Vec<CreatureId>> {
        self.tribes_creatures.get(tribe_id)
    }
}

#[derive(Debug)]
pub enum WorldChange {
    Structure(AbsoluteWorldPoint, StructureChange),
    Creature(CreatureId, CreatureChange),
}

#[derive(Debug)]
pub enum StructureChange {
    Set(Option<Structure>),
    SetOwned(StructureOwn),
}
