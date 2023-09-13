use crate::state::WorldArea;

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
pub struct AbsoluteWorldRowI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
pub struct AbsoluteWorldColI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
pub struct AbsoluteWorldPoint(pub AbsoluteWorldRowI, pub AbsoluteWorldColI);

impl AbsoluteWorldPoint {
    pub fn row_i(&self) -> &AbsoluteWorldRowI {
        &self.0
    }

    pub fn col_i(&self) -> &AbsoluteWorldColI {
        &self.1
    }
}

//.....?????
#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
pub struct RelativeWorldRowI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
pub struct RelativeWorldColI(pub isize);

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
pub struct RelativeWorldPoint(pub RelativeWorldRowI, pub RelativeWorldColI);

impl RelativeWorldPoint {
    pub fn row_i(&self) -> &RelativeWorldRowI {
        &self.0
    }

    pub fn col_i(&self) -> &RelativeWorldColI {
        &self.1
    }

    pub fn from_absolute(point: AbsoluteWorldPoint, reference: &WorldArea) -> Self {
        Self(
            RelativeWorldRowI(point.row_i().0 - reference.start().row_i().0),
            RelativeWorldColI(point.col_i().0 - reference.start().col_i().0),
        )
    }
}
