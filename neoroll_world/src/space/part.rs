use std::collections::HashMap;

use crate::{
    entity::{
        creature::{Creature, CreatureId},
        floor::Floor,
        ground::Ground,
        structure::Structure,
    },
    space::RelativeWorldPoint,
};

use super::{area::WorldArea, layer::CompositeLayer, patch::NewLayers, AbsoluteWorldPoint};

pub struct WorldPart {
    layers: LayersPart,
    creatures: HashMap<CreatureId, Creature>,
    area: WorldArea,
}

impl WorldPart {
    pub fn new(
        layers: LayersPart,
        creatures: HashMap<CreatureId, Creature>,
        area: WorldArea,
    ) -> Self {
        Self {
            layers,
            creatures,
            area,
        }
    }

    pub fn empty() -> Self {
        Self::new(
            LayersPart::new(
                CompositeLayer::empty(),
                CompositeLayer::empty(),
                CompositeLayer::empty(),
            ),
            HashMap::new(),
            WorldArea::new(AbsoluteWorldPoint::zero(), 0, 0),
        )
    }

    fn index(&self, point: &AbsoluteWorldPoint) -> usize {
        let relative_point = RelativeWorldPoint::from_absolute(point, &self.area);
        let row_i = relative_point.row_i().0;
        let col_i = relative_point.col_i().0;
        assert!(row_i >= 0);
        assert!(col_i >= 0);
        let row_i = row_i as usize;
        let col_i = col_i as usize;

        let i = row_i * self.area.columns() + col_i;
        assert!(i < self.layers.grounds().len());
        i
    }

    pub fn grounds(&self) -> Vec<(AbsoluteWorldPoint, &Option<Ground>)> {
        let mut grounds = vec![];

        for point in self.area().points() {
            grounds.push((point, self.ground(&point)));
        }

        grounds
    }

    pub fn ground(&self, point: &AbsoluteWorldPoint) -> &Option<Ground> {
        // Outside
        if !self.area.contains(point) {
            return &None;
        }

        self.layers.grounds().get(self.index(point))
    }

    pub fn floors(&self) -> Vec<(AbsoluteWorldPoint, &Option<Floor>)> {
        let mut floors = vec![];

        for point in self.area().points() {
            floors.push((point, self.floor(&point)));
        }

        floors
    }

    pub fn floor(&self, point: &AbsoluteWorldPoint) -> &Option<Floor> {
        // Outside
        if !self.area.contains(point) {
            return &None;
        }

        self.layers.floors().get(self.index(point))
    }

    pub fn structures(&self) -> Vec<(AbsoluteWorldPoint, &Option<Structure>)> {
        let mut structures = vec![];

        for point in self.area().points() {
            structures.push((point, self.structure(&point)));
        }

        structures
    }

    pub fn structure(&self, point: &AbsoluteWorldPoint) -> &Option<Structure> {
        // Outside
        if !self.area.contains(point) {
            return &None;
        }

        self.layers.structures().get(self.index(point))
    }

    pub fn creatures(&self) -> &HashMap<CreatureId, Creature> {
        &self.creatures
    }

    pub fn creature(&self, id: &CreatureId) -> Option<&Creature> {
        self.creatures.get(id)
    }

    pub fn area(&self) -> &WorldArea {
        &self.area
    }

    pub fn switch(&mut self, new: NewLayers, area: WorldArea) {
        let mut grounds = vec![];
        let mut floors = vec![];
        let mut structures = vec![];

        for point in area.points() {
            grounds.push(
                self.ground(&point)
                    .clone()
                    .or_else(|| new.ground(&point).cloned()),
            );
            floors.push(
                self.floor(&point)
                    .clone()
                    .or_else(|| new.floor(&point).cloned()),
            );
            structures.push(
                self.structure(&point)
                    .clone()
                    .or_else(|| new.structure(&point).cloned()),
            );
        }

        self.layers = LayersPart::new(
            CompositeLayer::new(grounds),
            CompositeLayer::new(floors),
            CompositeLayer::new(structures),
        );
        self.creatures = new
            .creatures()
            .iter()
            .map(|c| (*c.id(), c.clone()))
            .collect::<HashMap<CreatureId, Creature>>();
        self.area = area;
    }
}

pub struct LayersPart {
    grounds: CompositeLayer<Ground>,
    floors: CompositeLayer<Floor>,
    structures: CompositeLayer<Structure>,
}

impl LayersPart {
    pub fn new(
        grounds: CompositeLayer<Ground>,
        floors: CompositeLayer<Floor>,
        structures: CompositeLayer<Structure>,
    ) -> Self {
        Self {
            grounds,
            floors,
            structures,
        }
    }

    pub fn grounds(&self) -> &CompositeLayer<Ground> {
        &self.grounds
    }

    pub fn floors(&self) -> &CompositeLayer<Floor> {
        &self.floors
    }

    pub fn structures(&self) -> &CompositeLayer<Structure> {
        &self.structures
    }
}
