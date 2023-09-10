use crate::tile::RegionTile;

pub struct Region {
    type_: RegionTile,
}

pub struct Regions {
    value: Vec<Region>,
}

pub struct World {
    regions: Regions,
}
