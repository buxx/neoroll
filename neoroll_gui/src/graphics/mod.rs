pub mod map;
pub mod resolver;
pub mod tileset;
pub mod world;

pub const REGION_TILE_WIDTH: usize = 16;
pub const REGION_TILE_HEIGHT: usize = 16;

pub struct TileName(String);

pub struct AlphaByScale {
    limit: f32,
    from: f32,
    invert: bool,
}

impl AlphaByScale {
    const MAP_LIMIT: f32 = 1.5;
    const WORLD_LIMIT: f32 = 2.0;

    pub fn world() -> Self {
        Self {
            limit: Self::WORLD_LIMIT,
            from: Self::MAP_LIMIT,
            invert: false,
        }
    }

    pub fn map() -> Self {
        Self {
            limit: Self::MAP_LIMIT,
            from: Self::WORLD_LIMIT,
            invert: true,
        }
    }

    pub fn display(&self, scale: f32) -> bool {
        if scale > self.limit {
            return self.invert;
        }

        !self.invert
    }

    pub fn alpha(&self, scale: f32) -> f32 {
        ((self.limit - scale) / (self.limit - self.from)).clamp(0., 1.)
    }
}
