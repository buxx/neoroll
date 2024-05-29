use crate::entity::{floor::Floor, ground::Ground, structure::Structure};
use serde::{Deserialize, Serialize};

use super::{layer::Layers, AbsoluteWorldPoint};

// TODO: rename in World ?
#[derive(Deserialize, Serialize)]
pub struct World {
    layers: Layers,
    lines: usize,
    columns: usize,
}

impl World {
    pub fn new(layers: Layers, lines: usize, columns: usize) -> Self {
        Self {
            layers,
            lines,
            columns,
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
}

impl Default for World {
    fn default() -> Self {
        Self {
            layers: Default::default(),
            lines: Default::default(),
            columns: Default::default(),
        }
    }
}
