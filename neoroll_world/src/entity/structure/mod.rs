use crate::gameplay::{
    build::Buildable,
    material::{Material, Resource},
    CollectType, Quantity,
};

use super::Filled;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Structure {
    Nothing,
    BigLeafTree,
    FruitTree(Filled),
    Campfire,
    Storage,
}

// FIXME BS NOW: refactor filled/collect_quantity/etc code
// FIXME BS NOW: find an "mapping" way to automatically match CollectType / Resources, etc (and do not have multiples matches)
impl Structure {
    pub fn hide(&self) -> bool {
        match self {
            Structure::Nothing
            | Structure::BigLeafTree
            | Structure::FruitTree(_)
            | Structure::Campfire
            | Structure::Storage => false,
        }
    }

    pub fn filled(&self) -> Option<&Filled> {
        match self {
            Structure::Nothing
            | Structure::BigLeafTree
            | Structure::Campfire
            | Structure::Storage => None,

            Structure::FruitTree(filled) => Some(filled),
        }
    }

    pub fn collect_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Structure::Nothing
                | Structure::BigLeafTree
                | Structure::Campfire
                | Structure::Storage => None,

                Structure::FruitTree(_) => Some(Quantity(1000)),
            },
            CollectType::RawFlint => match self {
                Structure::Nothing
                | Structure::BigLeafTree
                | Structure::Campfire
                | Structure::Storage
                | Structure::FruitTree(_) => None,
            },
        }
    }

    pub fn maximum_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Structure::Nothing
                | Structure::BigLeafTree
                | Structure::Campfire
                | Structure::Storage => None,

                Structure::FruitTree(_) => Some(Quantity(25000)),
            },
            CollectType::RawFlint => match self {
                Structure::Nothing
                | Structure::BigLeafTree
                | Structure::Campfire
                | Structure::Storage
                | Structure::FruitTree(_) => None,
            },
        }
    }

    pub fn collectable(&self, type_: CollectType) -> Option<&Filled> {
        match type_ {
            CollectType::Food => match self {
                Structure::Nothing
                | Structure::BigLeafTree
                | Structure::Campfire
                | Structure::Storage => None,
                Structure::FruitTree(filled) => Some(filled),
            },
            CollectType::RawFlint => match self {
                Structure::Nothing
                | Structure::BigLeafTree
                | Structure::Campfire
                | Structure::Storage
                | Structure::FruitTree(_) => None,
            },
        }
    }

    pub fn collect_material(&self, type_: CollectType) -> Option<Material> {
        match type_ {
            CollectType::Food => match self {
                Structure::Nothing
                | Structure::BigLeafTree
                | Structure::Campfire
                | Structure::Storage => None,

                Structure::FruitTree(_) => Some(Material::Resource(Resource::Food)),
            },
            CollectType::RawFlint => match self {
                Structure::Nothing
                | Structure::BigLeafTree
                | Structure::Campfire
                | Structure::Storage
                | Structure::FruitTree(_) => None,
            },
        }
    }

    pub fn with_filled(&self, filled: Filled) -> Structure {
        match self {
            Structure::Nothing
            | Structure::BigLeafTree
            | Structure::Campfire
            | Structure::Storage => self.clone(),

            Structure::FruitTree(_) => Structure::FruitTree(filled),
        }
    }

    pub fn reduced(&self, type_: CollectType) -> (Structure, Quantity) {
        match self {
            Structure::Nothing
            | Structure::BigLeafTree
            | Structure::Campfire
            | Structure::Storage => (self.clone(), Quantity(0)),

            Structure::FruitTree(filled) => {
                let maximum_quantity = self
                    .maximum_quantity(type_)
                    .expect("Structure with reduce must own a maximum quantity");
                let collect_quantity = self
                    .collect_quantity(type_)
                    .expect("Structure with reduce must own a collect quantity");
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

impl From<Buildable> for Structure {
    fn from(value: Buildable) -> Self {
        match value {
            Buildable::Campfire => Structure::Campfire,
            Buildable::Storage => Structure::Storage,
        }
    }
}
