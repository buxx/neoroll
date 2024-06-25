use crate::gameplay::{material::Material, Quantity};

pub enum JobRequirement {
    Material(Material, Quantity),
}
