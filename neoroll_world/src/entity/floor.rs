use crate::gameplay::{
    material::{Material, Resource},
    CollectType, Quantity,
};

use super::Filled;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Floor {
    Nothing,
    ShortGrass,
    FruitBush(Filled),
}

// TODO: trait for collect/reduced/etc
impl Floor {
    pub fn hide(&self) -> bool {
        match self {
            Floor::Nothing => false,
            Floor::ShortGrass => true,
            Floor::FruitBush(_) => true,
        }
    }

    pub fn filled(&self) -> Option<&Filled> {
        match self {
            Floor::Nothing | Floor::ShortGrass => None,

            Floor::FruitBush(filled) => Some(filled),
        }
    }

    pub fn collect_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Floor::Nothing | Floor::ShortGrass => None,

                Floor::FruitBush(_) => Some(Quantity(500)),
            },
            CollectType::RawFlint => match self {
                Floor::Nothing | Floor::ShortGrass | Floor::FruitBush(_) => None,
            },
        }
    }

    pub fn collectable(&self, type_: CollectType) -> Option<&Filled> {
        match type_ {
            CollectType::Food => match self {
                Floor::Nothing | Floor::ShortGrass => None,

                Floor::FruitBush(filled) => Some(filled),
            },
            CollectType::RawFlint => match self {
                Floor::Nothing | Floor::ShortGrass | Floor::FruitBush(_) => None,
            },
        }
    }

    pub fn collect_material(&self, type_: CollectType) -> Option<Material> {
        match type_ {
            CollectType::Food => match self {
                Floor::Nothing | Floor::ShortGrass => None,

                Floor::FruitBush(_) => Some(Material::Resource(Resource::Food)),
            },
            CollectType::RawFlint => match self {
                Floor::Nothing | Floor::ShortGrass | Floor::FruitBush(_) => None,
            },
        }
    }

    pub fn maximum_quantity(&self, type_: CollectType) -> Option<Quantity> {
        match type_ {
            CollectType::Food => match self {
                Floor::Nothing | Floor::ShortGrass => None,

                Floor::FruitBush(_) => Some(Quantity(2000)),
            },
            CollectType::RawFlint => match self {
                Floor::Nothing | Floor::ShortGrass | Floor::FruitBush(_) => None,
            },
        }
    }

    pub fn with_filled(&self, filled: Filled) -> Floor {
        match self {
            Floor::Nothing | Floor::ShortGrass => self.clone(),

            Floor::FruitBush(_) => Floor::FruitBush(filled),
        }
    }

    pub fn reduced(&self, type_: CollectType) -> (Floor, Quantity) {
        match self {
            Floor::Nothing | Floor::ShortGrass => (self.clone(), Quantity(0)),

            Floor::FruitBush(filled) => {
                let maximum_quantity = self
                    .maximum_quantity(type_)
                    .expect("Floor with reduce must own a maximum quantity");
                let collect_quantity = self
                    .collect_quantity(type_)
                    .expect("Floor with reduce must own a collect quantity");
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

    pub fn detail_string(&self) -> String {
        match self {
            Floor::Nothing => "Nothing".to_string(),
            Floor::ShortGrass => "Short grass".to_string(),
            Floor::FruitBush(_) => "Fruit bush".to_string(),
        }
    }
}
