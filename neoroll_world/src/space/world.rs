use glam::Vec2;
use pathfinding::prelude::astar;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::{
    entity::{
        creature::{Creature, CreatureChange, CreatureId},
        floor::Floor,
        ground::Ground,
        structure::Structure,
    },
    gameplay::{
        material::Material,
        tribe::{structure::StructureOwn, TribeId},
        CollectType, Quantity,
    },
    space::{layer::Layers, AbsoluteWorldPoint},
    utils::Direction,
};
use serde::{Deserialize, Serialize};

use super::{AbsoluteWorldColI, AbsoluteWorldRowI};

#[derive(Deserialize, Serialize, Default)]
pub struct World {
    layers: Layers,
    lines: usize,
    columns: usize,
    creatures: HashMap<CreatureId, Creature>,
    tribes_creatures: HashMap<TribeId, Vec<CreatureId>>, // TODO: feel like it should be in `Game` ...
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

    pub fn material(&self, point: &AbsoluteWorldPoint) -> Option<&Vec<(Material, Quantity)>> {
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
        Some(self.layers.materials().get(i))
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

    pub fn set_floor(&mut self, point: AbsoluteWorldPoint, floor: Floor) {
        let row_i = point.row_i().0 as usize;
        let col_i = point.col_i().0 as usize;

        let i = row_i * self.columns + col_i;
        self.layers.floors_mut().set(i, floor);
    }

    pub fn add_material(
        &mut self,
        point: AbsoluteWorldPoint,
        material: Material,
        quantity: Quantity,
    ) {
        let row_i = point.row_i().0 as usize;
        let col_i = point.col_i().0 as usize;
        let i = row_i * self.columns + col_i;

        let materials = self.layers.materials_mut().get_mut(i);
        if let Some((_, quantity_)) = materials.iter_mut().find(|(m, _)| m == &material) {
            quantity_.0 += quantity.0;
        } else {
            materials.push((material, quantity));
        }
    }

    pub fn contains(&self, point: &AbsoluteWorldPoint) -> bool {
        point.0 .0 >= 0
            && point.1 .0 >= 0
            && point.0 .0 < self.lines as isize
            && point.1 .0 < self.columns as isize
    }

    // FIXME BS NOW: used for humans, but need to have separate code for humans !
    pub fn tribe_creature_ids(&self, tribe_id: &TribeId) -> Option<&Vec<CreatureId>> {
        self.tribes_creatures.get(tribe_id)
    }

    pub fn tribe_creatures(&self, tribe_id: &TribeId) -> Vec<&Creature> {
        self.tribe_creature_ids(tribe_id)
            .unwrap_or(&vec![])
            .iter()
            .map(|i| self.creatures().get(i).expect("Id just given"))
            .collect()
    }

    pub fn layers(&self) -> &Layers {
        &self.layers
    }

    pub fn layers_mut(&mut self) -> &mut Layers {
        &mut self.layers
    }

    pub fn can_collect(&self, point: &AbsoluteWorldPoint, collect_type: CollectType) -> bool {
        self.structure(point)
            .as_ref()
            .and_then(|s| s.collectable(collect_type).map(|f| !f.is_empty()))
            .unwrap_or(false)
            || self
                .floor(point)
                .and_then(|s| s.collectable(collect_type).map(|f| !f.is_empty()))
                .unwrap_or(false)
            || self
                .ground(point)
                .and_then(|s| s.collectable(collect_type).map(|f| !f.is_empty()))
                .unwrap_or(false)
    }

    pub fn materials_on(
        &self,
        point: &AbsoluteWorldPoint,
        filter: Option<Material>,
    ) -> Vec<&(Material, Quantity)> {
        // FIXME: something to refactor (see .floor(), .structure() etc)
        if point.row_i().0 >= self.lines as isize
            || point.row_i().0 < 0
            || point.col_i().0 >= self.columns as isize
            || point.col_i().0 < 0
        {
            return vec![];
        }

        let row_i = point.row_i().0 as usize;
        let col_i = point.col_i().0 as usize;

        let i = row_i * self.columns + col_i;

        let mut found = vec![];
        let materials = self.layers().materials().get(i);

        match filter {
            Some(filter_) => {
                return materials
                    .iter()
                    .filter(|(m, _)| m == &filter_)
                    .collect::<Vec<&(Material, Quantity)>>()
            }
            None => found.extend(materials),
        }

        found
    }

    pub fn find_path(
        &self,
        from: &AbsoluteWorldPoint,
        to: &AbsoluteWorldPoint,
    ) -> Option<(Vec<AbsoluteWorldPoint>, i32)> {
        astar(
            from,
            |p| self.successors(p),
            |p| Vec2::from(p).distance(Vec2::from(to)) as i32,
            |p| p == to,
        )
    }

    fn successors(&self, from: &AbsoluteWorldPoint) -> Vec<(AbsoluteWorldPoint, i32)> {
        let mut successors = vec![];

        for direction in Direction::iter() {
            let (mod_row, mod_col) = direction.modifier();
            let new_row_i = from.0 .0 + mod_row;
            let new_col_i = from.1 .0 + mod_col;
            let new_point =
                AbsoluteWorldPoint(AbsoluteWorldRowI(new_row_i), AbsoluteWorldColI(new_col_i));

            if !self.can_walk(&new_point) {
                continue;
            }

            successors.push((new_point, 1));
        }

        successors
    }

    pub fn can_walk(&self, point: &AbsoluteWorldPoint) -> bool {
        // Don't care ifd outside map
        if point.row_i().0 < 0
            || point.col_i().0 < 0
            || point.col_i().0 > self.columns as isize
            || point.row_i().0 > self.lines as isize
        {
            return false;
        }

        if let Some(Ground::FreshWater) = self.ground(point) {
            return false;
        }

        true
    }
}

#[derive(Debug)]
pub enum WorldChange {
    Structure(AbsoluteWorldPoint, StructureChange),
    Floor(AbsoluteWorldPoint, FloorChange),
    Material(AbsoluteWorldPoint, MaterialChange),
    Creature(CreatureId, CreatureChange),
}

#[derive(Debug)]
pub enum StructureChange {
    Set(Option<Structure>),
    SetOwned(StructureOwn),
}

#[derive(Debug)]
pub enum FloorChange {
    Set(Floor),
}

#[derive(Debug)]
pub enum MaterialChange {
    Add(Material, Quantity),
}
