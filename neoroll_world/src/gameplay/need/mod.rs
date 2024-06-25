use super::{job::Job, material::Material, Quantity};

#[derive(Debug, Clone, PartialEq)]
pub enum Need {
    MaterialInStorages(Material, Quantity),
}
