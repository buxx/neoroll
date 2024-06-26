use neoroll_world::gameplay::{
    material::{Material, Resource},
    target::{Target, TargetId},
};

pub struct TribeSettings {
    targets: Vec<(TargetId, Target)>,
}

impl TribeSettings {
    pub fn targets(&self) -> &Vec<(TargetId, Target)> {
        &self.targets
    }

    pub fn targets_mut(&mut self) -> &mut Vec<(TargetId, Target)> {
        &mut self.targets
    }
}

impl Default for TribeSettings {
    fn default() -> Self {
        Self {
            targets: vec![(
                TargetId::new(),
                Target::KeepStock(Material::Resource(Resource::Food), Default::default()).default(),
            )],
        }
    }
}
