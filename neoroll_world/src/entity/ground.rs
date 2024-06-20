use serde::{Deserialize, Serialize};

use crate::gameplay::{
    material::{Material, Resource},
    CollectType, Quantity,
};

use super::Filled;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub enum Ground {
    FreshWater,
    Soil,
    SoilFlint(Filled),
}

impl Ground {
    pub fn filled(&self) -> Option<&Filled> {
        match self {
            Ground::FreshWater | Ground::Soil => None,

            Ground::SoilFlint(filled) => Some(filled),
        }
    }

    pub fn collect_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Ground::FreshWater | Ground::Soil | Ground::SoilFlint(_) => None,
            },
            CollectType::RawFlint => match self {
                Ground::FreshWater | Ground::Soil => None,
                Ground::SoilFlint(_) => Some(Quantity(5)),
            },
        }
    }

    pub fn collectable(&self, type_: CollectType) -> Option<&Filled> {
        match type_ {
            CollectType::Food => match self {
                Ground::FreshWater | Ground::Soil | Ground::SoilFlint(_) => None,
            },
            CollectType::RawFlint => match self {
                Ground::FreshWater | Ground::Soil => None,
                Ground::SoilFlint(filled) => Some(filled),
            },
        }
    }

    pub fn collect_material(&self, type_: CollectType) -> Option<Material> {
        match type_ {
            CollectType::Food => match self {
                Ground::FreshWater | Ground::Soil | Ground::SoilFlint(_) => None,
            },
            CollectType::RawFlint => match self {
                Ground::FreshWater | Ground::Soil => None,
                Ground::SoilFlint(_) => Some(Material::Resource(Resource::RawFlint)),
            },
        }
    }

    pub fn maximum_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Ground::FreshWater | Ground::Soil | Ground::SoilFlint(_) => None,
            },
            CollectType::RawFlint => match self {
                Ground::FreshWater | Ground::Soil => None,

                Ground::SoilFlint(_) => Some(Quantity(1000)),
            },
        }
    }

    pub fn with_filled(&self, filled: Filled) -> Ground {
        match self {
            Ground::FreshWater | Ground::Soil => self.clone(),

            Ground::SoilFlint(_) => Ground::SoilFlint(filled),
        }
    }

    pub fn reduced(&self, type_: CollectType) -> (Ground, Quantity) {
        match self {
            Ground::FreshWater | Ground::Soil => (self.clone(), Quantity(0)),

            Ground::SoilFlint(filled) => {
                let maximum_quantity = self
                    .maximum_quantity(type_)
                    .expect("Ground with reduce must own a maximum quantity");
                let collect_quantity = self
                    .collect_quantity(type_)
                    .expect("Ground with reduce must own a collect quantity");
                let current_quantity: u64 =
                    (maximum_quantity.0 as f32 * (filled.0 as f32 / 255.)) as u64;
                let collectable_quantity_ = collect_quantity.0.min(current_quantity);
                let new_quantity_ = current_quantity - collectable_quantity_;
                let new_filled_ = ((new_quantity_ as f32 / maximum_quantity.0 as f32) * 255.) as u8;

                let new_filled = Filled::new(new_filled_);

                (
                    self.with_filled(new_filled),
                    Quantity(collectable_quantity_),
                )
            }
        }
    }
}
