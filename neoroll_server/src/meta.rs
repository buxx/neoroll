use neoroll_world::space::AbsoluteWorldPoint;

#[derive(Debug, Default)]
pub struct MetaState {
    reserved_moves: Vec<AbsoluteWorldPoint>,
}

impl MetaState {
    pub fn clear(&mut self) {
        self.reserved_moves.clear();
    }

    /// Try to book a point. If point is not booked, its returned.
    pub fn book(&mut self, point: &AbsoluteWorldPoint) -> Option<AbsoluteWorldPoint> {
        if self.reserved_moves.contains(point) {
            return None;
        }

        self.reserved_moves.push(*point);
        Some(*point)
    }
}
