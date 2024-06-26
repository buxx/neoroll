use neoroll_world::gameplay::{
    material::{Material, Resource},
    target::{Target, TargetId},
};

pub struct TribeSettings {
    targets: Vec<(TargetId, TargetSetting)>,
}

impl TribeSettings {
    pub fn targets(&self) -> &Vec<(TargetId, TargetSetting)> {
        &self.targets
    }

    pub fn targets_mut(&mut self) -> &mut Vec<(TargetId, TargetSetting)> {
        &mut self.targets
    }
}

impl Default for TribeSettings {
    fn default() -> Self {
        Self {
            targets: vec![(
                TargetId::new(),
                TargetSetting::new(
                    Target::KeepStock(Material::Resource(Resource::Food), Default::default())
                        .default(),
                    1,
                ),
            )],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TargetSetting {
    target: Target,
    priority: usize,
}

impl TargetSetting {
    pub fn new(target: Target, priority: usize) -> Self {
        Self { target, priority }
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn priority(&self) -> usize {
        self.priority
    }
}
