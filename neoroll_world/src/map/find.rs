use std::mem;

use strum::IntoEnumIterator;

use crate::{
    entity::{floor::Floor, ground::Ground, structure::Structure, Filled},
    gameplay::{material::Material, CollectType},
    space::{world::World, AbsoluteWorldPoint},
    utils::Direction,
};

pub struct AroundTileFinder<'a> {
    start: AbsoluteWorldPoint,
    world: &'a World,
    ground: Option<Ground>,
    floor: Option<Floor>,
    structure: Option<Structure>,
    material: Option<Material>,
    collect: Option<CollectType>,
    max_distance: isize,
}

impl<'a> AroundTileFinder<'a> {
    pub fn new(world: &'a World, start: AbsoluteWorldPoint) -> Self {
        Self {
            start,
            world,
            ground: Default::default(),
            floor: Default::default(),
            structure: Default::default(),
            material: Default::default(),
            collect: Default::default(),
            max_distance: 100,
        }
    }

    pub fn ground(mut self, value: Option<Ground>) -> Self {
        self.ground = value;
        self
    }

    pub fn floor(mut self, value: Option<Floor>) -> Self {
        self.floor = value;
        self
    }

    pub fn structure(mut self, value: Option<Structure>) -> Self {
        self.structure = value;
        self
    }

    pub fn material(mut self, value: Option<Material>) -> Self {
        self.material = value;
        self
    }

    pub fn collect(mut self, value: Option<CollectType>) -> Self {
        self.collect = value;
        self
    }

    pub fn max_distance(mut self, value: isize) -> Self {
        self.max_distance = value;
        self
    }

    pub fn search(&self) -> Option<AbsoluteWorldPoint> {
        let mut distance: isize = 1;

        if self.test_point(&self.start).is_some() {
            return Some(self.start);
        }

        while distance <= self.max_distance {
            for direction in Direction::iter() {
                let modifier = direction.modifier();
                let modifier = (modifier.0 * distance, modifier.1 * distance);
                if let Some(point) = self.test_point(&self.start.apply(modifier)) {
                    return Some(point);
                }
            }
            distance += 1;
        }

        None
    }

    fn test_point(&self, point: &AbsoluteWorldPoint) -> Option<AbsoluteWorldPoint> {
        if let (Some(point_ground), Some(expected_ground)) =
            (self.world.ground(point), &self.ground)
        {
            if mem::discriminant(point_ground) == mem::discriminant(expected_ground) {
                return Some(*point);
            }
        }

        if let (Some(point_floor), Some(expected_floor)) = (self.world.floor(point), &self.floor) {
            if mem::discriminant(point_floor) == mem::discriminant(expected_floor) {
                return Some(*point);
            }
        }

        if let (Some(point_structure), Some(expected_structure)) =
            (self.world.structure(point), &self.structure)
        {
            if mem::discriminant(point_structure) == mem::discriminant(expected_structure) {
                return Some(*point);
            }
        }

        if let (Some(point_material), Some(expected_material)) =
            (self.world.material(point), &self.material)
        {
            for (point_material, _) in point_material {
                if mem::discriminant(point_material) == mem::discriminant(expected_material) {
                    return Some(*point);
                }
            }
        }

        if let Some(expected_collect) = &self.collect {
            if let Some(point_ground) = self.world.ground(point) {
                if !point_ground
                    .collectable(*expected_collect)
                    .unwrap_or(&Filled(0))
                    .is_empty()
                {
                    return Some(*point);
                }
            }

            if let Some(point_floor) = self.world.floor(point) {
                if !point_floor
                    .collectable(*expected_collect)
                    .unwrap_or(&Filled(0))
                    .is_empty()
                {
                    return Some(*point);
                }
            }

            if let Some(point_structure) = self.world.structure(point) {
                if !point_structure
                    .collectable(*expected_collect)
                    .unwrap_or(&Filled(0))
                    .is_empty()
                {
                    return Some(*point);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entity::Filled;
    use crate::space::layer::Layers;
    use crate::space::layer::{CompositeLayer, FilledLayer};
    use crate::space::{AbsoluteWorldColI, AbsoluteWorldRowI};
    use rstest::*;

    #[rstest]
    #[case(None, None, None, None, None, None)]
    #[case(
        Some(Ground::Soil),
        None,
        None,
        None,
        None,
        Some(AbsoluteWorldPoint(AbsoluteWorldRowI(0), AbsoluteWorldColI(0)))
    )]
    #[case(
        None,
        None,
        None,
        None,
        Some(CollectType::Food),
        Some(AbsoluteWorldPoint(AbsoluteWorldRowI(2), AbsoluteWorldColI(2)))
    )]
    fn test_around_tile_finder(
        #[case] ground: Option<Ground>,
        #[case] floor: Option<Floor>,
        #[case] structure: Option<Structure>,
        #[case] material: Option<Material>,
        #[case] collect: Option<CollectType>,
        #[case] expected: Option<AbsoluteWorldPoint>,
    ) {
        // GIVEN

        let layers = Layers::new(
            FilledLayer::new(vec![
                Ground::Soil,
                Ground::Soil,
                Ground::Soil,
                //
                Ground::Soil,
                Ground::Soil,
                Ground::Soil,
                //
                Ground::Soil,
                Ground::Soil,
                Ground::Soil,
            ]),
            FilledLayer::new(vec![
                Floor::Nothing,
                Floor::Nothing,
                Floor::Nothing,
                //
                Floor::Nothing,
                Floor::Nothing,
                Floor::Nothing,
                //
                Floor::Nothing,
                Floor::Nothing,
                Floor::Nothing,
            ]),
            CompositeLayer::new(vec![
                None,
                None,
                None,
                //
                None,
                None,
                None,
                //
                None,
                None,
                Some(Structure::FruitTree(Filled(255))),
            ]),
            FilledLayer::new(vec![
                vec![],
                vec![],
                vec![],
                //
                vec![],
                vec![],
                vec![],
                //
                vec![],
                vec![],
                vec![],
            ]),
        );
        let world = World::new(layers, 3, 3, vec![]);
        let start = AbsoluteWorldPoint(AbsoluteWorldRowI(0), AbsoluteWorldColI(0));
        let finder = AroundTileFinder::new(&world, start)
            .ground(ground)
            .floor(floor)
            .structure(structure)
            .material(material)
            .collect(collect);

        // WHEN
        let found = finder.search();

        // THEN
        assert_eq!(expected, found);
    }
}
