use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{Quantity, Weight};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Material {
    Resource(Resource),
}
impl Material {
    pub fn quantity_string(&self, quantity: &Quantity) -> String {
        let (unit, divide) = match self {
            Material::Resource(resource) => match resource {
                Resource::Food => ("Kg", 1_000), // wrote in g
                Resource::RawFlint => ("u", 1),  // wrote in u
                Resource::Branches => ("m³", 1), // wrote in m³
            },
        };

        format!("{}{}", quantity.0 / divide, unit)
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Material::Resource(resource) => f.write_str(&resource.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Resource {
    Food,
    RawFlint,
    Branches,
    // CarvedFlint,
}

impl Resource {
    pub fn weight(self, quantity: &Quantity) -> Weight {
        let factor = match self {
            Resource::Food => 1,
            Resource::RawFlint => 1000,
            Resource::Branches => 1,
        };
        Weight(quantity.0 * factor)
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::Food => f.write_str("Food"),
            Resource::RawFlint => f.write_str("Raw Flint"),
            Resource::Branches => f.write_str("Branches"),
            // Resource::CarvedFlint => f.write_str("Carved Flint"),
        }
    }
}
