use neoroll_world::gameplay::{
    material::{Material, Resource},
    target::{Target, TargetQuantity},
    Quantity,
};

pub struct TribeSettings {
    targets: Vec<Target>,
}

impl TribeSettings {
    pub fn targets(&self) -> &[Target] {
        &self.targets
    }
}

impl Default for TribeSettings {
    fn default() -> Self {
        Self {
            targets: vec![Target::KeepStock(
                Material::Resource(Resource::Food),
                TargetQuantity::PerHuman(Quantity(2000)),
            )],
        }
    }
}
