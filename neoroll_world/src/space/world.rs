use std::collections::HashMap;

use crate::entity::{
    creature::{Creature, CreatureChange, CreatureId},
    floor::Floor,
    ground::Ground,
    structure::Structure,
};
use serde::{Deserialize, Serialize};

use super::{layer::Layers, AbsoluteWorldPoint};

#[derive(Deserialize, Serialize, Default)]
pub struct World {
    layers: Layers,
    lines: usize,
    columns: usize,
    creatures: HashMap<CreatureId, Creature>,
}

impl World {
    pub fn new(layers: Layers, lines: usize, columns: usize, creatures: Vec<Creature>) -> Self {
        Self {
            layers,
            lines,
            columns,
            creatures: creatures
                .into_iter()
                .map(|c| (*c.id(), c))
                .collect::<HashMap<CreatureId, Creature>>(),
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

    pub fn creature(&self, id: &CreatureId) -> Option<&Creature> {
        self.creatures.get(id)
    }

    pub fn apply(&mut self, change: WorldChange) {
        match change {
            WorldChange::Creature(id, change) => match change {
                CreatureChange::New(creature) => {
                    self.creatures.insert(*creature.id(), creature);
                }
                CreatureChange::SetPoint(point) => {
                    // FIXME BS NOW: think about how to propagate to client(s)
                    self.creatures.get_mut(&id).unwrap().set_position(point)
                }
            },
        }
    }
}

pub enum WorldChange {
    Creature(CreatureId, CreatureChange),
}
