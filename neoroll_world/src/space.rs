#[derive(Debug, Eq, PartialEq)]
pub struct RowI(pub isize);

#[derive(Debug, Eq, PartialEq)]
pub struct ColI(pub isize);

#[derive(Debug, Eq, PartialEq)]
pub struct RegionCoordinate(pub RowI, pub ColI);
