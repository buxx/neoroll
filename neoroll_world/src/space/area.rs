use super::{AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI};

#[derive(Debug, Default, Clone)]
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
}
