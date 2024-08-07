use std::fmt::Display;

use crate::gameplay::build::Buildable;

use super::Filled;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Structure {
    // FIXME: Nothing ? Why Option in World ?
    Nothing,
    BigLeafTree(Filled),       // Branches
    FruitTree(Filled, Filled), // Branches, Fruits
    Campfire,
    Storage,
}

impl Structure {
    pub fn hide(&self) -> bool {
        match self {
            Structure::Nothing
            | Structure::BigLeafTree(_)
            | Structure::FruitTree(_, _)
            | Structure::Campfire
            | Structure::Storage => false,
        }
    }

    // pub fn collect_quantity(&self, type_: CollectType) -> Option<Quantity> {
    //     match type_ {
    //         CollectType::Food => match self {
    //             Structure::Nothing
    //             | Structure::BigLeafTree(_)
    //             | Structure::Campfire
    //             | Structure::Storage => None,

    //             Structure::FruitTree(_) => Some(Quantity(1000)),
    //         },
    //         CollectType::RawFlint => match self {
    //             Structure::Nothing
    //             | Structure::BigLeafTree(_)
    //             | Structure::Campfire
    //             | Structure::Storage
    //             | Structure::FruitTree(_, _) => None,
    //         },
    //         CollectType::Branches => match self {
    //             Structure::Nothing | Structure::Campfire | Structure::Storage => None,
    //             Structure::BigLeafTree | Structure::FruitTree(_) => Some(Quantity(5_000)),
    //         },
    //     }
    // }

    // pub fn maximum_quantity(&self, type_: CollectType) -> Option<Quantity> {
    //     match type_ {
    //         CollectType::Food => match self {
    //             Structure::Nothing
    //             | Structure::BigLeafTree
    //             | Structure::Campfire
    //             | Structure::Storage => None,

    //             Structure::FruitTree(_) => Some(Quantity(25000)),
    //         },
    //         CollectType::RawFlint => match self {
    //             Structure::Nothing
    //             | Structure::BigLeafTree
    //             | Structure::Campfire
    //             | Structure::Storage
    //             | Structure::FruitTree(_) => None,
    //         },
    //         CollectType::Branches => match self {
    //             Structure::Nothing | Structure::Campfire | Structure::Storage => None,
    //             Structure::BigLeafTree | Structure::FruitTree(_) => Some(Quantity(1_000_000)),
    //         },
    //     }
    // }

    // pub fn collectable(&self, type_: CollectType) -> Option<&Filled> {
    //     match type_ {
    //         CollectType::Food => match self {
    //             Structure::Nothing
    //             | Structure::BigLeafTree
    //             | Structure::Campfire
    //             | Structure::Storage => None,
    //             Structure::FruitTree(filled) => Some(filled),
    //         },
    //         CollectType::RawFlint => match self {
    //             Structure::Nothing
    //             | Structure::BigLeafTree
    //             | Structure::Campfire
    //             | Structure::Storage
    //             | Structure::FruitTree(_) => None,
    //         },
    //     }
    // }

    // pub fn collect_material(&self, type_: CollectType) -> Option<Material> {
    //     match type_ {
    //         CollectType::Food => match self {
    //             Structure::Nothing
    //             | Structure::BigLeafTree
    //             | Structure::Campfire
    //             | Structure::Storage => None,

    //             Structure::FruitTree(_) => Some(Material::Resource(Resource::Food)),
    //         },
    //         CollectType::RawFlint => match self {
    //             Structure::Nothing
    //             | Structure::BigLeafTree
    //             | Structure::Campfire
    //             | Structure::Storage
    //             | Structure::FruitTree(_) => None,
    //         },
    //     }
    // }

    // pub fn with_filled(&self, filled: Filled) -> Structure {
    //     match self {
    //         Structure::Nothing
    //         | Structure::BigLeafTree
    //         | Structure::Campfire
    //         | Structure::Storage => self.clone(),

    //         Structure::FruitTree(_) => Structure::FruitTree(filled),
    //     }
    // }

    // pub fn reduced(&self, type_: CollectType) -> (Structure, Quantity) {
    //     // if let Some(collect) = self.collect(type_) {
    //     //     let maximum_quantity = collect
    //     //         .maximum(type_)
    //     //         .expect("Structure with reduce must own a maximum quantity");
    //     //     let collect_quantity = self
    //     //         .collect_quantity(type_)
    //     //         .expect("Structure with reduce must own a collect quantity");
    //     //     let current_quantity: u64 =
    //     //         (maximum_quantity.0 as f32 * (filled.0 as f32 / 255.)) as u64;
    //     //     let collectable_quantity_ = collect_quantity.0.min(current_quantity);
    //     //     let new_quantity_ = current_quantity - collectable_quantity_;
    //     //     let new_filled_ = ((new_quantity_ as f32 / maximum_quantity.0 as f32) * 255.) as u8;

    //     //     let new_filled = Filled::new(new_filled_);

    //     //     (
    //     //         self.with_filled(new_filled),
    //     //         Quantity(collectable_quantity_),
    //     //     )
    //     // } else {
    //     //     (self.clone(), Quantity(0))
    //     // }

    //     // match self {
    //     //     Structure::Nothing
    //     //     | Structure::BigLeafTree(_)
    //     //     | Structure::Campfire
    //     //     | Structure::Storage => (self.clone(), Quantity(0)),

    //     //     Structure::FruitTree(branches_filled, food_filled) => {
    //     //         let maximum_quantity = self
    //     //             .maximum_quantity(type_)
    //     //             .expect("Structure with reduce must own a maximum quantity");
    //     //         let collect_quantity = self
    //     //             .collect_quantity(type_)
    //     //             .expect("Structure with reduce must own a collect quantity");
    //     //         let current_quantity: u64 =
    //     //             (maximum_quantity.0 as f32 * (filled.0 as f32 / 255.)) as u64;
    //     //         let collectable_quantity_ = collect_quantity.0.min(current_quantity);
    //     //         let new_quantity_ = current_quantity - collectable_quantity_;
    //     //         let new_filled_ = ((new_quantity_ as f32 / maximum_quantity.0 as f32) * 255.) as u8;

    //     //         let new_filled = Filled::new(new_filled_);

    //     //         (
    //     //             self.with_filled(new_filled),
    //     //             Quantity(collectable_quantity_),
    //     //         )
    //     //     }
    //     // }
    // }

    pub fn detail_string(&self) -> String {
        match self {
            Structure::Nothing => "Nothing".to_string(),
            Structure::BigLeafTree(_) => "Big leaf tree".to_string(),
            Structure::FruitTree(_, _) => "Fruit tree".to_string(),
            Structure::Campfire => "Campfire".to_string(),
            Structure::Storage => "Storage".to_string(),
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

impl Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Structure::Nothing => f.write_str("Nothing"),
            Structure::BigLeafTree(_) => f.write_str("BigLeafTree"),
            Structure::FruitTree(_, _) => f.write_str("FruitTree"),
            Structure::Campfire => f.write_str("Campfire"),
            Structure::Storage => f.write_str("Storage"),
        }
    }
}
