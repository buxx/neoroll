use std::fmt::Display;

use enum_display::EnumDisplay;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Material {
    Resource(Resource),
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Material::Resource(resource) => f.write_str(&resource.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, EnumDisplay)]
pub enum Resource {
    Food,
}
