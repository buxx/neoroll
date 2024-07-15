use crate::entity::{floor::Floor, ground::Ground, structure::Structure, Filled};

use super::{
    material::{Material, Resource},
    CollectType, Quantity,
};

pub struct Collect<'a> {
    material: Material,
    maximum: Quantity,
    iteration: Quantity,
    filled: &'a Filled,
}

impl<'a> Collect<'a> {
    pub fn new(
        material: Material,
        maximum: Quantity,
        iteration: Quantity,
        filled: &'a Filled,
    ) -> Self {
        Self {
            material,
            maximum,
            iteration,
            filled,
        }
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn maximum(&self) -> &Quantity {
        &self.maximum
    }

    pub fn iteration(&self) -> &Quantity {
        &self.iteration
    }

    pub fn filled(&self) -> &Filled {
        self.filled
    }
}

pub trait IntoCollect: Clone {
    fn collect(&self, type_: CollectType) -> Option<Collect>;
    fn with_filled(&self, type_: CollectType, new_filled: Filled) -> Self;

    fn material(&self, type_: CollectType) -> Option<Material> {
        self.collect(type_).map(|c| c.material())
    }

    fn reduced(&self, type_: CollectType) -> (Self, Quantity) {
        if let Some(collect) = self.collect(type_) {
            let maximum = collect.maximum();
            let iteration = collect.iteration();
            let filled = collect.filled();
            let current: u64 = (maximum.0 as f32 * (filled.0 as f32 / 255.)) as u64;
            let collectable = iteration.0.min(current);
            let new_quantity_ = current - collectable;
            let new_filled_raw = ((new_quantity_ as f32 / maximum.0 as f32) * 255.) as u8;
            let new_filled = Filled::new(new_filled_raw);

            (self.with_filled(type_, new_filled), Quantity(collectable))
        } else {
            (self.clone(), Quantity(0))
        }
    }
}

impl IntoCollect for Ground {
    fn collect(&self, type_: CollectType) -> Option<Collect> {
        match self {
            Ground::FreshWater => None,
            Ground::Soil => None,
            Ground::SoilFlint(raw_flint_filled) => match type_ {
                CollectType::RawFlint => Some(Collect::new(
                    Material::Resource(Resource::RawFlint),
                    Quantity(1000),
                    Quantity(5),
                    raw_flint_filled,
                )),
                _ => None,
            },
        }
    }

    fn with_filled(&self, type_: CollectType, new_filled: Filled) -> Self {
        match type_ {
            CollectType::Food => self.clone(),
            CollectType::RawFlint => match self {
                Ground::FreshWater | Ground::Soil => self.clone(),
                Ground::SoilFlint(_) => Ground::SoilFlint(new_filled),
            },
            CollectType::Branches => self.clone(),
        }
    }
}

impl IntoCollect for Floor {
    fn collect(&self, type_: CollectType) -> Option<Collect> {
        match self {
            Floor::Nothing => None,
            Floor::ShortGrass => None,
            Floor::FruitBush(food_filled) => match type_ {
                CollectType::Food => Some(Collect::new(
                    Material::Resource(Resource::Food),
                    Quantity(2000),
                    Quantity(500),
                    food_filled,
                )),
                _ => None,
            },
        }
    }

    fn with_filled(&self, type_: CollectType, new_filled: Filled) -> Self {
        match type_ {
            CollectType::Food => match self {
                Floor::Nothing => self.clone(),
                Floor::ShortGrass => self.clone(),
                Floor::FruitBush(_) => Floor::FruitBush(new_filled),
            },
            CollectType::RawFlint => self.clone(),
            CollectType::Branches => self.clone(),
        }
    }
}

impl IntoCollect for Structure {
    fn collect(&self, type_: CollectType) -> Option<Collect> {
        match self {
            Structure::Nothing => None,
            Structure::FruitTree(branches_filled, food_filled) => match type_ {
                CollectType::Food => Some(Collect::new(
                    Material::Resource(Resource::Food),
                    Quantity(25000),
                    Quantity(1000),
                    food_filled,
                )),
                CollectType::Branches => Some(Collect::new(
                    Material::Resource(Resource::Branches),
                    Quantity(1),
                    Quantity(1),
                    branches_filled,
                )),
                _ => None,
            },
            Structure::BigLeafTree(branches_filled) => match type_ {
                CollectType::Branches => Some(Collect::new(
                    Material::Resource(Resource::Branches),
                    Quantity(1),
                    Quantity(1),
                    branches_filled,
                )),
                _ => None,
            },
            Structure::Campfire => None,
            Structure::Storage => None,
        }
    }

    fn with_filled(&self, type_: CollectType, new_filled: Filled) -> Self {
        match type_ {
            CollectType::Food => match self {
                Structure::Nothing => todo!(),
                Structure::BigLeafTree(_) => self.clone(),
                Structure::FruitTree(branches_filled, _) => {
                    Structure::FruitTree(branches_filled.clone(), new_filled)
                }
                Structure::Campfire => self.clone(),
                Structure::Storage => self.clone(),
            },
            CollectType::RawFlint => self.clone(),
            CollectType::Branches => match self {
                Structure::Nothing => todo!(),
                Structure::BigLeafTree(_) => self.clone(),
                Structure::FruitTree(_, food_filled) => {
                    Structure::FruitTree(new_filled, food_filled.clone())
                }
                Structure::Campfire => self.clone(),
                Structure::Storage => self.clone(),
            },
        }
    }
}
