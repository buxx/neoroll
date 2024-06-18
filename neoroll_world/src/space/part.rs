use std::collections::HashMap;

use crate::{
    entity::{
        creature::{CreatureId, PartialCreature},
        floor::Floor,
        ground::Ground,
        structure::Structure,
    },
    gameplay::{material::Material, Quantity},
    space::RelativeWorldPoint,
};

use super::{area::WorldArea, layer::CompositeLayer, patch::NewLayers, AbsoluteWorldPoint};

pub struct WorldPart {
    layers: LayersPart,
    creatures: HashMap<CreatureId, PartialCreature>,
    area: WorldArea,
}

impl WorldPart {
    pub fn new(
        layers: LayersPart,
        creatures: HashMap<CreatureId, PartialCreature>,
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

    #[allow(clippy::type_complexity)]
    pub fn materials(&self) -> Vec<(AbsoluteWorldPoint, &Option<Vec<(Material, Quantity)>>)> {
        let mut materials = vec![];

        for point in self.area().points() {
            materials.push((point, self.material(&point)));
        }

        materials
    }

    pub fn material(&self, point: &AbsoluteWorldPoint) -> &Option<Vec<(Material, Quantity)>> {
        // Outside
        if !self.area.contains(point) {
            return &None;
        }

        self.layers.materials().get(self.index(point))
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

    pub fn creatures(&self) -> &HashMap<CreatureId, PartialCreature> {
        &self.creatures
    }

    pub fn creature(&self, id: &CreatureId) -> Option<&PartialCreature> {
        self.creatures.get(id)
    }

    pub fn creature_mut(&mut self, id: &CreatureId) -> Option<&mut PartialCreature> {
        self.creatures.get_mut(id)
    }

    pub fn area(&self) -> &WorldArea {
        &self.area
    }

    pub fn set_structure(&mut self, point: &AbsoluteWorldPoint, structure: Option<Structure>) {
        let i = self.index(point);
        self.layers.structures_mut().set(i, structure);
    }

    pub fn set_materials(
        &mut self,
        point: &AbsoluteWorldPoint,
        materials: Vec<(Material, Quantity)>,
    ) {
        let i = self.index(point);
        self.layers.materials_mut().set(i, Some(materials));
    }

    pub fn set_floor(&mut self, point: &AbsoluteWorldPoint, floor: Floor) {
        let i = self.index(point);
        // TODO: Why LayersPart own only CompositeLayer unlike World ?
        self.layers.floors_mut().set(i, Some(floor));
    }

    pub fn add_creature(&mut self, creature: PartialCreature) {
        self.creatures.insert(*creature.id(), creature);
    }

    pub fn switch(&mut self, new: NewLayers, area: WorldArea) {
        let mut grounds = vec![];
        let mut floors = vec![];
        let mut structures = vec![];
        let mut materials = vec![];

        for point in area.points() {
            // dbg!(&new.material(&point));
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
            materials.push(
                self.material(&point)
                    .clone()
                    .or_else(|| new.material(&point).cloned()),
            );
        }

        self.layers = LayersPart::new(
            CompositeLayer::new(grounds),
            CompositeLayer::new(floors),
            CompositeLayer::new(structures),
            CompositeLayer::new(materials),
        );
        self.creatures = new
            .creatures()
            .iter()
            .map(|c| (*c.id(), c.clone()))
            .collect::<HashMap<CreatureId, PartialCreature>>();
        self.area = area;
    }

    pub fn clear(&mut self) {
        self.layers = LayersPart::default();
        self.creatures = Default::default();
        self.area = WorldArea::zero();
    }
}

#[derive(Default)]
pub struct LayersPart {
    grounds: CompositeLayer<Ground>,
    floors: CompositeLayer<Floor>,
    structures: CompositeLayer<Structure>,
    materials: CompositeLayer<Vec<(Material, Quantity)>>,
}

impl LayersPart {
    pub fn new(
        grounds: CompositeLayer<Ground>,
        floors: CompositeLayer<Floor>,
        structures: CompositeLayer<Structure>,
        materials: CompositeLayer<Vec<(Material, Quantity)>>,
    ) -> Self {
        Self {
            grounds,
            floors,
            structures,
            materials,
        }
    }

    pub fn grounds(&self) -> &CompositeLayer<Ground> {
        &self.grounds
    }

    pub fn grounds_mut(&mut self) -> &mut CompositeLayer<Ground> {
        &mut self.grounds
    }

    pub fn floors(&self) -> &CompositeLayer<Floor> {
        &self.floors
    }

    pub fn floors_mut(&mut self) -> &mut CompositeLayer<Floor> {
        &mut self.floors
    }

    pub fn structures(&self) -> &CompositeLayer<Structure> {
        &self.structures
    }

    pub fn structures_mut(&mut self) -> &mut CompositeLayer<Structure> {
        &mut self.structures
    }

    pub fn materials(&self) -> &CompositeLayer<Vec<(Material, Quantity)>> {
        &self.materials
    }

    pub fn materials_mut(&mut self) -> &mut CompositeLayer<Vec<(Material, Quantity)>> {
        &mut self.materials
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorldPartMessage {
    Structure(AbsoluteWorldPoint, WorldPartStructureMessage),
    Floor(AbsoluteWorldPoint, WorldPartFloorMessage),
    Creature(CreatureId, WorldPartCreatureMessage),
    Material(AbsoluteWorldPoint, WorldPartMaterialMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorldPartStructureMessage {
    Set(Option<Structure>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorldPartFloorMessage {
    Set(Floor),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorldPartCreatureMessage {
    New(PartialCreature),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorldPartMaterialMessage {
    Set(Vec<(Material, Quantity)>),
}
