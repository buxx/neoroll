use anyhow::{bail, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    entity::ground::Ground,
    space::{world::World, AbsoluteWorldPoint},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    Front,
    FrontLeft,
    Left,
    RearLeft,
    Rear,
    RearRight,
    Right,
    FrontRight,
}

impl Direction {
    pub fn counter_clockwise(from: &Self) -> Vec<Self> {
        match from {
            Self::Front => vec![
                Self::FrontLeft,
                Self::Left,
                Self::RearLeft,
                Self::Rear,
                Self::RearRight,
                Self::Right,
                Self::FrontRight,
            ],
            Self::FrontLeft => vec![
                Self::Left,
                Self::RearLeft,
                Self::Rear,
                Self::RearRight,
                Self::Right,
                Self::FrontRight,
                Self::Front,
            ],
            Self::Left => vec![
                Self::RearLeft,
                Self::Rear,
                Self::RearRight,
                Self::Right,
                Self::FrontRight,
                Self::Front,
                Self::FrontLeft,
            ],
            Self::RearLeft => vec![
                Self::Rear,
                Self::RearRight,
                Self::Right,
                Self::FrontRight,
                Self::Front,
                Self::FrontLeft,
                Self::Left,
            ],
            Self::Rear => vec![
                Self::RearRight,
                Self::Right,
                Self::FrontRight,
                Self::Front,
                Self::FrontLeft,
                Self::Left,
                Self::RearLeft,
            ],
            Self::RearRight => vec![
                Self::Right,
                Self::FrontRight,
                Self::Front,
                Self::FrontLeft,
                Self::Left,
                Self::RearLeft,
                Self::Rear,
            ],
            Self::Right => vec![
                Self::FrontRight,
                Self::Front,
                Self::FrontLeft,
                Self::Left,
                Self::RearLeft,
                Self::Rear,
                Self::RearRight,
            ],
            Self::FrontRight => vec![
                Self::Front,
                Self::FrontLeft,
                Self::Left,
                Self::RearLeft,
                Self::Rear,
                Self::RearRight,
                Self::Right,
            ],
        }
        .into_iter()
        .collect()
    }

    pub fn clockwise(from: &Self) -> Vec<Self> {
        Self::counter_clockwise(from).into_iter().rev().collect()
    }

    /// (row, col)
    pub fn modifier(&self) -> (isize, isize) {
        match self {
            Self::Front => (-1, 0),
            Self::FrontLeft => (-1, -1),
            Self::Left => (0, -1),
            Self::RearLeft => (1, -1),
            Self::Rear => (1, 0),
            Self::RearRight => (1, 1),
            Self::Right => (0, 1),
            Self::FrontRight => (-1, 1),
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Self::Front => Self::Left,
            Self::FrontLeft => Self::RearLeft,
            Self::Left => Self::Rear,
            Self::RearLeft => Self::RearRight,
            Self::Rear => Self::Right,
            Self::RearRight => Self::FrontRight,
            Self::Right => Self::Front,
            Self::FrontRight => Self::FrontLeft,
        }
    }

    fn relative(reference: &AbsoluteWorldPoint, point: &AbsoluteWorldPoint) -> Option<Self> {
        match (point.0 .0 - reference.0 .0, point.1 .0 - reference.1 .0) {
            (-1, 0) => Some(Self::Front),
            (-1, -1) => Some(Self::FrontLeft),
            (0, -1) => Some(Self::Left),
            (1, -1) => Some(Self::RearLeft),
            (1, 0) => Some(Self::Rear),
            (1, 1) => Some(Self::RearRight),
            (0, 1) => Some(Self::Right),
            (-1, 1) => Some(Self::FrontRight),
            _ => None,
        }
    }
}

pub struct BlindFoldedMazesResolver<'a> {
    world: &'a World,
    coasts: &'a Vec<AbsoluteWorldPoint>,
    start_in_water: bool,
}

impl<'a> BlindFoldedMazesResolver<'a> {
    pub fn new(world: &'a World, coasts: &'a Vec<AbsoluteWorldPoint>) -> Self {
        Self {
            world,
            coasts,
            start_in_water: true,
        }
    }

    pub fn start_in_water(mut self, value: bool) -> Self {
        self.start_in_water = value;
        self
    }

    pub fn resolve_all(&self) -> Vec<Vec<AbsoluteWorldPoint>> {
        let mut lines = vec![];
        let mut excluded: Vec<AbsoluteWorldPoint> = vec![];

        while let Some(start) = self.coasts.iter().find(|c| {
            let in_found_lines = lines.iter().flatten().any(|p| &p == c);
            let in_excluded = excluded.contains(c);
            !in_found_lines && !in_excluded
        }) {
            if let Ok(mut walker) =
                BlindFoldedMazeWalker::new(self.world, self.coasts, start, self.start_in_water)
            {
                match walker.resolve() {
                    Ok(line) => lines.push(line),
                    Err(_) => {
                        excluded.extend(walker.wall);
                    }
                };
            } else {
                excluded.push(*start);
            }
        }

        lines
    }
}

pub struct BlindFoldedMazeWalker<'a> {
    coasts: &'a Vec<AbsoluteWorldPoint>,
    wall: Vec<AbsoluteWorldPoint>,
    handled: AbsoluteWorldPoint,
    foot: AbsoluteWorldPoint,
    looking: Direction,
    history: Vec<(AbsoluteWorldPoint, Direction)>,
}

impl<'a> BlindFoldedMazeWalker<'a> {
    pub fn new(
        world: &'a World,
        coasts: &'a Vec<AbsoluteWorldPoint>,
        start: &AbsoluteWorldPoint,
        start_in_water: bool,
    ) -> Result<Self> {
        // Find start place and looking direction
        if let Some((foot, looking)) = Direction::iter()
            .map(|direction| (start.next(&direction), direction.left()))
            .find(|(try_foot, try_looking)| {
                // Foot must not be on the coast
                !coasts.contains(try_foot)
                // Foot must be inside the lake (so in water)
                && (!start_in_water || world.ground(try_foot) == Some(&Ground::FreshWater))
                // Tile at left must be start (because left handled)
                && &try_foot.next(&try_looking.left()) == start
            })
        {
            Ok(Self {
                coasts,
                wall: vec![*start],
                handled: *start,
                foot,
                looking,
                history: vec![],
            })
        } else {
            bail!("Can't make a maze from this tile (probably too thin)")
        }
    }

    pub fn resolve(&mut self) -> Result<Vec<AbsoluteWorldPoint>> {
        let (start_foot, start_handled) = (self.foot, self.handled);

        loop {
            let (new_foot, new_looking, new_handled) = self.move_()?;

            if start_foot == new_foot && start_handled == new_handled
                || self.history.contains(&(new_foot, new_looking))
            {
                break;
            }

            if !self.wall.contains(&new_handled) {
                self.wall.push(new_handled);
            }
            self.foot = new_foot;
            self.handled = new_handled;
            self.looking = new_looking;
            self.history.push((new_foot, new_looking));
        }

        Ok(self.wall.clone())
    }

    fn move_(&self) -> Result<(AbsoluteWorldPoint, Direction, AbsoluteWorldPoint)> {
        let direction_from_wall = Direction::relative(&self.handled, &self.foot)
            .expect("Algorithm imply foot are neighbor of handled wall");

        // Try to rotate around handled wall (by counter-clockwise)
        let directions = Direction::counter_clockwise(&direction_from_wall);
        let try_direction = directions
            .first()
            .expect("There is always a next direction");
        let try_tile = self.handled.next(try_direction);
        let is_walkable = !self.coasts.contains(&try_tile);

        // Fine, we can move on this tile
        if is_walkable {
            let new_foot = try_tile;
            let new_looking = try_direction.left();
            return Ok((new_foot, new_looking, self.handled));
        }

        // If not possible, change handled wall tile on adjacent handled tile (clockwise)
        let directions = Direction::clockwise(&self.looking);
        for try_direction in directions {
            let left_tile = self.foot.next(&try_direction.left());
            let left_tile_is_adjacent = Direction::iter()
                .map(|direction| self.handled.next(&direction))
                .filter(|around| self.coasts.contains(around))
                .any(|around| around == left_tile);

            // We found a new wall to put our hand
            if left_tile_is_adjacent {
                let new_looking = try_direction;
                let new_handled = left_tile;
                return Ok((self.foot, new_looking, new_handled));
            }
        }

        bail!("Maze seems not resolvable")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map::builder::MapBuilder;
    use crate::space::{AbsoluteWorldColI, AbsoluteWorldRowI};
    use crate::tests::str_map::WorldFromStrBuilder;
    use rstest::*;

    #[rstest]
    #[case(
        "000
         010
         000", (1, 1), vec![(1, 1)])]
    #[case(
        "0000
         0110
         0000", (1, 1), vec![(1, 1), (1, 2)])]
    #[case(
        "111
         101
         111", (0, 0), vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)])]
    #[case(
        "010
         110
         000", (1, 0), vec![(1, 0), (1, 1), (0, 1)])]
    fn test_blind_folded_maze_walker(
        #[case] map: &str,
        #[case] start: (isize, isize),
        #[case] expected: Vec<(isize, isize)>,
    ) {
        // Given
        let world = WorldFromStrBuilder::new(map).build();
        let coasts = MapBuilder::new(&world).coasts();
        let start = AbsoluteWorldPoint(AbsoluteWorldRowI(start.0), AbsoluteWorldColI(start.1));

        // When
        let line = BlindFoldedMazeWalker::new(&world, &coasts, &start, false)
            .unwrap()
            .resolve()
            .unwrap();

        // Then
        let result = line
            .iter()
            .map(|p| (p.0 .0, p.1 .0))
            .collect::<Vec<(isize, isize)>>();
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(
        "000
         010
         000", vec![vec![(1, 1)]], false)]
    #[case(
        "111
         101
         111", vec![vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)]], false)]
    #[case(
        "1110
         1010
         1110
         0000
         0110
         0110",
         vec![
           vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)],
           vec![(4, 1), (5, 1), (5, 2), (4, 2)]
         ], false
    )]
    #[case(
        "111
         110
         100
         100",
         vec![vec![(0, 0), (1, 0), (2, 0), (3, 0), (1, 1), (0, 1), (0, 2)]], false
    )]
    #[case(
        "10
         11",
         vec![vec![(0, 0), (1, 0), (1, 1)]], false
    )]
    // #[case(
    //     "000000000000000000000000000
    //      000000000000001110000000000
    //      000000000000001111110001000
    //      000000000000011111101001100
    //      000000000000011111101111100
    //      000000000000011111101111100
    //      000000000000111111110111110
    //      000000000000011111111111110
    //      000000000000111111111111100
    //      000000110000111111111111100
    //      000011111100111111111111000
    //      000111111101111111111111000
    //      001111111000111111111111000
    //      001111111111111111111111000
    //      001111111111111111111110000
    //      011111111111111111111110000",
    //      vec![vec![]]
    // )]
    // #[case(
    //     "000000000000000000000000000000000000000000000000000000000
    //      000000000000000000000000000000000000000000000000000000000
    //      000000000000000000000000000000001100000000000000000000000
    //      000000000000000000000000011111111110000000000000000000000
    //      000000000000000000000000111111111111100000000000000000000
    //      000000000000000000000000111111111111111100000000000000000
    //      000000000000000000000001111111111111111100100000000000000
    //      000000000000000000000001111111111111111111100000000000000
    //      000000000000000000000001111111111111111111111000000000000
    //      000000000000000000000011111111111111111111111111111000000
    //      000000000000000000000011111111111001111111111111111111000
    //      000000000000000011111111111111111111111111111111111111100
    //      000000001111111111111111111111111111111111111111111111100
    //      000000011111111111111111111111111111111111111111111111110
    //      000000111111111111111111111111111111110011111111111111110
    //      000000111111111111111111111111100011100001111111111111100
    //      000000111111111111111111111111111111110011111111111111100
    //      000000111111111111111111111111111111111111111111111111000
    //      000000111111111111111111111111111111111111111111111100000
    //      000001111111111111111111111111111111111111111111111110000
    //      000011111111111001111111111111111111111111111111111100000
    //      000001111111111000011111111111111111111111111111011000000
    //      000011111111111100000111111111111111111111111110000000000
    //      000001111111110000000001111111111111111100111110000000000
    //      000001111111100000000000011111011111111000000000000000000
    //      000001111111100000000000000100001111111000000000000000000
    //      000001111111000000000000000000000000000000000000000000000
    //      000110111111100000000000000000000000000000000000000000000
    //      000011111111100000000000000000000000000000000000000000000
    //      000111111110100000000000000000000000000000000000000000000
    //      000111110000000000000000000000000000000000000000000000000
    //      001111111010000000000000000000000000000000000000000000000
    //      000011111100000000000000000000000000000000000000000000000
    //      010001111100000000000000000000000000000000000000000000000
    //      001110001011100000000000000000000000000000000000000000000
    //      001110000011000000000000000000000000000000000000000000000
    //      001100000111000000000000000000000000000000000000000000000
    //      001110000000000000000000000000000000000000000000000000000
    //      000100000000000000000000000000000000000000000000000000000
    //      000000000000000000000000000000000000000000000000000000000",
    //      vec![vec![]]
    // )]
    fn test_blind_folded_mazes_resolver(
        #[case] map: &str,
        #[case] expected: Vec<Vec<(isize, isize)>>,
        #[case] start_in_water: bool,
    ) {
        // Given
        let world = WorldFromStrBuilder::new(map).build();
        let coasts = MapBuilder::new(&world).coasts();

        // When
        let lakes = BlindFoldedMazesResolver::new(&world, &coasts)
            .start_in_water(start_in_water)
            .resolve_all();

        // Then
        let result = lakes
            .iter()
            .map(|lake| {
                lake.iter()
                    .map(|p| (p.0 .0, p.1 .0))
                    .collect::<Vec<(isize, isize)>>()
            })
            .collect::<Vec<Vec<(isize, isize)>>>();
        assert_eq!(result, expected)
    }
}
