use neoroll_world::entity::structure::Structure;

use crate::image::Illustration;

pub trait IntoIllustration {
    fn illustration(&self) -> Option<Illustration>;
}

impl IntoIllustration for Structure {
    fn illustration(&self) -> Option<Illustration> {
        match self {
            Structure::Nothing => None,
            Structure::BigLeafTree => Some(Illustration::ArtocarpusAltilis),
            Structure::FruitTree(_) => None,
            Structure::Campfire => None,
            Structure::Storage => None,
        }
    }
    
}