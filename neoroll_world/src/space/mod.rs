use crate::{
    map::{AbsoluteMapPoint, MAP_TILE_FACTOR},
    utils::Direction,
};

use self::area::WorldArea;

pub mod area;
pub mod layer;
pub mod part;
pub mod patch;
pub mod world;

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct AbsoluteWorldRowI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct AbsoluteWorldColI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct AbsoluteWorldPoint(pub AbsoluteWorldRowI, pub AbsoluteWorldColI);

impl AbsoluteWorldPoint {
    pub fn zero() -> Self {
        Self(AbsoluteWorldRowI(0), AbsoluteWorldColI(0))
    }

    pub fn row_i(&self) -> &AbsoluteWorldRowI {
        &self.0
    }

    pub fn col_i(&self) -> &AbsoluteWorldColI {
        &self.1
    }

    pub fn next(&self, direction: &Direction) -> Self {
        let modifier = direction.modifier();
        Self(
            AbsoluteWorldRowI(self.0 .0 + modifier.1 as isize),
            AbsoluteWorldColI(self.1 .0 + modifier.0 as isize),
        )
    }
}

impl From<AbsoluteMapPoint> for AbsoluteWorldPoint {
    fn from(val: AbsoluteMapPoint) -> Self {
        AbsoluteWorldPoint(
            AbsoluteWorldRowI(val.0 .0 * MAP_TILE_FACTOR as isize),
            AbsoluteWorldColI(val.1 .0 * MAP_TILE_FACTOR as isize),
        )
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct RelativeWorldRowI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct RelativeWorldColI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Hash)]
pub struct RelativeWorldPoint(pub RelativeWorldRowI, pub RelativeWorldColI);

impl RelativeWorldPoint {
    pub fn row_i(&self) -> &RelativeWorldRowI {
        &self.0
    }

    pub fn col_i(&self) -> &RelativeWorldColI {
        &self.1
    }

    pub fn from_absolute(point: &AbsoluteWorldPoint, reference: &WorldArea) -> Self {
        Self(
            RelativeWorldRowI(point.row_i().0 - reference.start().row_i().0),
            RelativeWorldColI(point.col_i().0 - reference.start().col_i().0),
        )
    }
}

#[cfg(test)]
mod test {
    use super::part::*;
    use super::*;
    use crate::entity::floor::*;
    use crate::entity::ground::*;
    use crate::space::area::*;
    use crate::space::layer::*;
    use crate::space::patch::*;
    use crate::space::world::*;
    use rstest::*;

    #[cfg(test)]
    #[fixture]
    fn entire_world() -> EntireWorld {
        use crate::space::layer::{FilledLayer, Layers};

        let lines = 5;
        let columns = 5;
        let grounds = (0..25).map(|_| Ground::Soil).collect();
        let floors = (0..25).map(|_| Floor::Nothing).collect();
        let structures = (0..25).map(|_| None).collect();
        EntireWorld::new(
            Layers::new(
                FilledLayer::new(grounds),
                FilledLayer::new(floors),
                CompositeLayer::new(structures),
            ),
            lines,
            columns,
        )
    }

    #[rstest]
    #[case(
        (0, 0),
        5,
        5,
        vec![
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
        ]
    )]
    #[case(
        (0, 0),
        2,
        5,
        vec![
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
        ]
    )]
    #[case(
        (0, 0),
        2,
        2,
        vec![
            1, 1,
            1, 1,
        ]
    )]
    #[case(
        (0, 0),
        6,
        6,
        vec![
            1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0,
        ]
    )]
    #[case(
        (-1, -1),
        5,
        5,
        vec![
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 1,
            0, 1, 1, 1, 1,
            0, 1, 1, 1, 1,
            0, 1, 1, 1, 1,
        ]
    )]
    fn test_world_part(
        entire_world: EntireWorld,
        #[case] start: (isize, isize),
        #[case] lines: usize,
        #[case] columns: usize,
        #[case] expected: Vec<usize>,
    ) {
        let (start_row, start_col) = start;
        let area = WorldArea::new(
            AbsoluteWorldPoint(AbsoluteWorldRowI(start_row), AbsoluteWorldColI(start_col)),
            lines,
            columns,
        );
        let mut world_part = WorldPart::empty();

        let new_layers = NewLayers::from_world_area(&entire_world, &area, &WorldArea::zero());
        world_part.switch(new_layers, area);

        let grounds: Vec<usize> = world_part
            .grounds()
            .iter()
            .map(|(_, r)| r.is_some() as usize)
            .collect();
        assert_eq!(grounds, expected)
    }
}
