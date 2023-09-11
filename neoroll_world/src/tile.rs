use strum_macros::Display;

#[derive(Debug, Hash, Copy, Clone, Display)]
pub enum RegionTile {
    GrassLand,
    Forest,
}
