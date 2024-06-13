pub mod build;
pub mod builder;
pub mod human;
pub mod material;
pub mod target;

use build::BuildGameState;
use human::HumanGameState;
use material::MaterialsState;
use neoroll_world::gameplay::tribe::TribeId;
use target::TargetsGameState;

use super::game::need::ComputedNeed;

#[derive(Debug, Clone, PartialEq)]
pub struct ClientGameState {
    tribe_id: TribeId,
    human: HumanGameState,
    build: BuildGameState,
    target: TargetsGameState,
    needs: Vec<ComputedNeed>,
    materials: MaterialsState,
}

impl ClientGameState {
    pub fn new(
        tribe_id: TribeId,
        human: HumanGameState,
        build: BuildGameState,
        target: TargetsGameState,
        needs: Vec<ComputedNeed>,
        materials: MaterialsState,
    ) -> Self {
        Self {
            tribe_id,
            human,
            build,
            target,
            needs,
            materials,
        }
    }

    pub fn tribe_id(&self) -> &TribeId {
        &self.tribe_id
    }

    pub fn human(&self) -> &HumanGameState {
        &self.human
    }

    pub fn build(&self) -> &BuildGameState {
        &self.build
    }

    pub fn target(&self) -> &TargetsGameState {
        &self.target
    }

    pub fn can_configure_targets(&self) -> bool {
        // For now, consider simply as this
        true
    }

    pub fn needs(&self) -> &[ComputedNeed] {
        &self.needs
    }

    pub fn materials(&self) -> &MaterialsState {
        &self.materials
    }
}
