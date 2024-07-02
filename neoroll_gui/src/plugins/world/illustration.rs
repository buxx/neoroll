use neoroll_world::{
    entity::{floor::Floor, ground::Ground, structure::Structure},
    gameplay::material::{Material, Resource},
};

use crate::image::Illustration;

pub trait IntoIllustration {
    fn illustration(&self) -> Option<Illustration>;
}

impl IntoIllustration for Structure {
    fn illustration(&self) -> Option<Illustration> {
        match self {
            Structure::Nothing => None,
            Structure::BigLeafTree => Some(Illustration::ArtocarpusAltilis),
            Structure::FruitTree(_) => Some(Illustration::ApricotTree),
            Structure::Campfire => Some(Illustration::Campfire),
            Structure::Storage => None,
        }
    }
}

impl IntoIllustration for Ground {
    fn illustration(&self) -> Option<Illustration> {
        match self {
            Ground::FreshWater => Some(Illustration::FreshWater1),
            Ground::Soil => Some(Illustration::Soil),
            Ground::SoilFlint(_) => Some(Illustration::FlintQuary),
        }
    }
}

impl IntoIllustration for Floor {
    fn illustration(&self) -> Option<Illustration> {
        match self {
            Floor::Nothing => None,
            Floor::ShortGrass => Some(Illustration::ShortAndDryGrass),
            Floor::FruitBush(_) => Some(Illustration::AbuluntuEmpetrumNigrum),
        }
    }
}

impl IntoIllustration for Material {
    fn illustration(&self) -> Option<Illustration> {
        match self {
            Material::Resource(resource) => match resource {
                Resource::Food => Some(Illustration::Fruits1),
                Resource::RawFlint => Some(Illustration::RawFlint1),
            },
        }
    }
}
