use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::space::AbsoluteWorldPoint;

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
    /// (row, col)
    pub fn modifier(&self) -> (i32, i32) {
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
}

pub struct BlindFoldedMazesResolver<'a> {
    coasts: &'a Vec<AbsoluteWorldPoint>,
}

impl<'a> BlindFoldedMazesResolver<'a> {
    pub fn new(coast: &'a Vec<AbsoluteWorldPoint>) -> Self {
        Self { coasts: coast }
    }

    pub fn resolve_all(&self) -> Vec<Vec<AbsoluteWorldPoint>> {
        let mut lines = vec![];

        while let Some(start) = self
            .coasts
            .iter()
            .find(|c| !lines.iter().flatten().any(|p| &p == c))
        {
            lines.push(BlindFoldedMazeWalker::new(self.coasts, start).resolve());
        }

        lines
    }
}

pub struct BlindFoldedMazeWalker<'a> {
    coast: &'a Vec<AbsoluteWorldPoint>,
    wall: Vec<AbsoluteWorldPoint>,
    left_hand: AbsoluteWorldPoint,
    foot_tile: AbsoluteWorldPoint,
    looking: Direction,
    history: Vec<(AbsoluteWorldPoint, Direction)>,
}

impl<'a> BlindFoldedMazeWalker<'a> {
    pub fn new(coast: &'a Vec<AbsoluteWorldPoint>, start: &AbsoluteWorldPoint) -> Self {
        // Find start place and looking direction
        let (foot_tile, looking) = [
            (start.next(&Direction::Front), Direction::Front.left()),
            (start.next(&Direction::Left), Direction::Left.left()),
            (start.next(&Direction::Right), Direction::Right.left()),
            (start.next(&Direction::Rear), Direction::Rear.left()),
        ]
        .into_iter()
        .find(|(possible_foot_tile, _)| !coast.contains(possible_foot_tile))
        .expect("Adjacent tile for foot must alway exist");

        Self {
            coast,
            wall: vec![*start],
            left_hand: *start,
            foot_tile,
            looking,
            history: vec![(foot_tile, looking)],
        }
    }

    pub fn resolve(&mut self) -> Vec<AbsoluteWorldPoint> {
        // Find adjacent foot tile (4) where left hand on same wall, or wall adjacent (8) wall
        while let Some((foot_tile, looking, wall_tile)) = self.find_new_position() {
            if !self.wall.contains(&wall_tile) {
                self.wall.push(wall_tile);
            }
            self.foot_tile = foot_tile;
            self.left_hand = wall_tile;
            self.looking = looking;
            self.history.push((foot_tile, looking));
        }

        self.wall.clone()
    }

    fn find_new_position(&self) -> Option<(AbsoluteWorldPoint, Direction, AbsoluteWorldPoint)> {
        // Test each move around current foot tile
        for test_foot_tile in Direction::iter()
            .map(|direction| self.foot_tile.next(&direction))
            // Tested foot tile must be walkable (not a wall)
            .filter(|p| !self.coast.contains(p))
        {
            // For each of this tested tile, test a looking direction
            for test_looking in Direction::iter() {
                // Left tile for this looking direction
                let left_tile = test_foot_tile.next(&test_looking.left());
                let left_tile_is_same = self.left_hand == left_tile;
                let left_tile_is_adjacent = Direction::iter()
                    .map(|direction| self.left_hand.next(&direction))
                    .filter(|around| self.coast.contains(around))
                    .any(|around| around == left_tile);
                let already_done = self.history.contains(&(test_foot_tile, test_looking));
                let is_new_tile = !self.wall.contains(&left_tile);

                if already_done {
                    continue;
                }

                // Move left hand on new wall tile or Move around current wall tile
                if (left_tile_is_adjacent && is_new_tile) || left_tile_is_same {
                    return Some((test_foot_tile, test_looking, left_tile));
                }
            }
        }

        None
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
         000", (1, 0), vec![(1, 0), (1, 1), (0, 1), (0, 0)])]
    fn test_blind_folded_maze_walker(
        #[case] map: &str,
        #[case] start: (isize, isize),
        #[case] expected: Vec<(isize, isize)>,
    ) {
        // Given
        let world = WorldFromStrBuilder::new(map).build();
        let coasts = MapBuilder::new(&world).coasts();
        let start = AbsoluteWorldPoint(AbsoluteWorldRowI(start.1), AbsoluteWorldColI(start.0));

        // When
        let line = BlindFoldedMazeWalker::new(&coasts, &start).resolve();

        // Then
        let result = line
            .iter()
            .map(|p| (p.0 .0, p.1 .0))
            .collect::<Vec<(isize, isize)>>();
        assert_eq!(result, expected)
    }

    #[rstest]
    // #[case(
    //     "000
    //      010
    //      000", vec![vec![(1, 1)]])]
    // #[case(
    //     "111
    //      101
    //      111", vec![vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)]])]
    // #[case(
    //     "1110
    //      1010
    //      1110
    //      0000
    //      0110
    //      0110",
    //      vec![
    //        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)],
    //        vec![(4, 1), (5, 1), (5, 2), (4, 2)]
    //      ]
    // )]
    #[case(
        "111
         110
         100
         100", 
         vec![
            vec![]
             ]
    )]
    fn test_blind_folded_mazes_resolver(
        #[case] map: &str,
        #[case] expected: Vec<Vec<(isize, isize)>>,
    ) {
        // Given
        let world = WorldFromStrBuilder::new(map).build();
        let coasts = MapBuilder::new(&world).coasts();

        // When
        let lakes = BlindFoldedMazesResolver::new(&coasts).resolve_all();

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
