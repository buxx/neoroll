use super::{AbsoluteMapColI, AbsoluteMapPoint, AbsoluteMapRowI};

#[derive(Debug, Default, Clone)]
pub struct MapArea {
    start: AbsoluteMapPoint,
    lines: usize,
    columns: usize,
}

impl MapArea {
    pub fn new(start: AbsoluteMapPoint, lines: usize, columns: usize) -> Self {
        Self {
            start,
            lines,
            columns,
        }
    }

    pub fn start(&self) -> AbsoluteMapPoint {
        self.start
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn start_row(&self) -> AbsoluteMapRowI {
        AbsoluteMapRowI(self.start().row_i().0)
    }

    pub fn end_row(&self) -> AbsoluteMapRowI {
        AbsoluteMapRowI(self.start_row().0 + self.lines() as isize)
    }

    pub fn start_col(&self) -> AbsoluteMapColI {
        AbsoluteMapColI(self.start().col_i().0)
    }

    pub fn end_col(&self) -> AbsoluteMapColI {
        AbsoluteMapColI(self.start_col().0 + self.columns() as isize)
    }

    pub fn rows(&self) -> Vec<AbsoluteMapRowI> {
        let from = self.start_row().0;
        let to = self.end_row().0;
        (from..to).map(AbsoluteMapRowI).collect()
    }

    pub fn cols(&self) -> Vec<AbsoluteMapColI> {
        let from = self.start_col().0;
        let to = self.end_col().0;
        (from..to).map(AbsoluteMapColI).collect()
    }

    pub fn points(&self) -> Vec<AbsoluteMapPoint> {
        let mut points = vec![];

        for row in self.rows() {
            for col in self.cols() {
                points.push(AbsoluteMapPoint(row, col));
            }
        }

        points
    }

    pub fn contains(&self, point: &AbsoluteMapPoint) -> bool {
        !(point.row_i().0 >= self.end_row().0
            || point.row_i().0 < self.start_row().0
            || point.col_i().0 >= self.end_col().0
            || point.col_i().0 < self.start_col().0)
    }

    pub fn zero() -> Self {
        Self {
            start: AbsoluteMapPoint(AbsoluteMapRowI(0), AbsoluteMapColI(0)),
            lines: 0,
            columns: 0,
        }
    }

    pub fn resize(&self, lines: isize, columns: isize) -> Self {
        Self {
            start: AbsoluteMapPoint(
                AbsoluteMapRowI(self.start.0 .0 - lines),
                AbsoluteMapColI(self.start.1 .0 - columns),
            ),
            lines: (self.lines as isize + lines * 2) as usize,
            columns: (self.columns as isize + columns * 2) as usize,
        }
    }
}
