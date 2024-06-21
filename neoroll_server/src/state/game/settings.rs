use std::collections::HashMap;

use neoroll_world::gameplay::{
    material::{Material, Resource},
    target::{Target, TargetId, TargetQuantity},
    Quantity,
};

pub struct TribeSettings {
    targets: HashMap<TargetId, Target>,
}

impl TribeSettings {
    pub fn targets(&self) -> &HashMap<TargetId, Target> {
        &self.targets
    }

    pub fn targets_mut(&mut self) -> &mut HashMap<TargetId, Target> {
        &mut self.targets
    }
}

impl Default for TribeSettings {
    fn default() -> Self {
        Self {
            targets: HashMap::from([(
                TargetId::default(),
                Target::KeepStock(
                    Material::Resource(Resource::Food),
                    TargetQuantity::PerHuman(Quantity(2000)),
                ),
            )]),
        }
    }
}
