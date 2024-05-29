use super::{AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WorldArea {
    start: AbsoluteWorldPoint,
    lines: usize,
    columns: usize,
}

impl WorldArea {
    pub fn new(start: AbsoluteWorldPoint, lines: usize, columns: usize) -> Self {
        Self {
            start,
            lines,
            columns,
        }
    }

    pub fn start(&self) -> AbsoluteWorldPoint {
        self.start
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn start_row(&self) -> AbsoluteWorldRowI {
        AbsoluteWorldRowI(self.start().row_i().0)
    }

    pub fn end_row(&self) -> AbsoluteWorldRowI {
        AbsoluteWorldRowI(self.start_row().0 + self.lines() as isize)
    }

    pub fn start_col(&self) -> AbsoluteWorldColI {
        AbsoluteWorldColI(self.start().col_i().0)
    }

    pub fn end_col(&self) -> AbsoluteWorldColI {
        AbsoluteWorldColI(self.start_col().0 + self.columns() as isize)
    }

    pub fn rows(&self) -> Vec<AbsoluteWorldRowI> {
        let from = self.start_row().0;
        let to = self.end_row().0;
        (from..to).map(AbsoluteWorldRowI).collect()
    }

    pub fn cols(&self) -> Vec<AbsoluteWorldColI> {
        let from = self.start_col().0;
        let to = self.end_col().0;
        (from..to).map(AbsoluteWorldColI).collect()
    }

    pub fn points(&self) -> Vec<AbsoluteWorldPoint> {
        let mut points = vec![];

        for row in self.rows() {
            for col in self.cols() {
                points.push(AbsoluteWorldPoint(row, col));
            }
        }

        points
    }

    pub fn contains(&self, point: &AbsoluteWorldPoint) -> bool {
        !(point.row_i().0 >= self.end_row().0
            || point.row_i().0 < self.start_row().0
            || point.col_i().0 >= self.end_col().0
            || point.col_i().0 < self.start_col().0)
    }

    pub fn zero() -> Self {
        Self {
            start: AbsoluteWorldPoint(AbsoluteWorldRowI(0), AbsoluteWorldColI(0)),
            lines: 0,
            columns: 0,
        }
    }

    pub fn resize(&self, lines: isize, columns: isize) -> Self {
        Self {
            start: AbsoluteWorldPoint(
                AbsoluteWorldRowI(self.start.0 .0 - lines),
                AbsoluteWorldColI(self.start.1 .0 - columns),
            ),
            lines: (self.lines as isize + lines * 2) as usize,
            columns: (self.columns as isize + columns * 2) as usize,
        }
    }

    pub fn include(&self, point: &AbsoluteWorldPoint) -> bool {
        point.0 .0 >= self.start.0 .0
            && point.1 .0 >= self.start.1 .0
            && point.0 .0 < self.start.0 .0 + self.lines as isize
            && point.1 .0 < self.start.1 .0 + self.columns as isize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case((-10, -10), 20, 20, 5, (-15, -15), 30, 30)]
    #[case((-10, -10), 20, 20, 100, (-110, -110), 220, 220)]
    fn test_world_area_resize(
        #[case] start: (isize, isize),
        #[case] lines: usize,
        #[case] columns: usize,
        #[case] resize: isize,
        #[case] new_start: (isize, isize),
        #[case] new_lines: usize,
        #[case] new_columns: usize,
    ) {
        let area = WorldArea::new(
            AbsoluteWorldPoint(AbsoluteWorldRowI(start.0), AbsoluteWorldColI(start.1)),
            lines,
            columns,
        );

        let new_area = area.resize(resize, resize);

        assert_eq!(
            new_area.start(),
            AbsoluteWorldPoint(
                AbsoluteWorldRowI(new_start.0),
                AbsoluteWorldColI(new_start.1)
            )
        );
        assert_eq!(new_area.lines(), new_lines);
        assert_eq!(new_area.columns(), new_columns);
    }
}
