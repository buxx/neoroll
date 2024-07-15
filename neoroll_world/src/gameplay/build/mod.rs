use crate::{
    entity::{floor::Floor, ground::Ground, structure::Structure},
    space::{world::World, AbsoluteWorldPoint},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Buildable {
    Campfire,
    Storage,
}

pub struct TryBuild<'a> {
    world: &'a World,
}

impl<'a> TryBuild<'a> {
    pub fn new(world: &'a World) -> Self {
        Self { world }
    }

    pub fn try_(
        &self,
        _buildable: &Buildable,
        point: &AbsoluteWorldPoint,
    ) -> Result<(), TryBuildError> {
        if !self.world.contains(point) {
            return Err(TryBuildError::OutsideWorld);
        }

        if let Some(ground) = self.world.ground(point) {
            match ground {
                Ground::FreshWater => {
                    return Err(TryBuildError::IncompatibleGround(ground.clone()))
                }
                Ground::Soil | Ground::SoilFlint(_) => {}
            }
        }

        if let Some(floor) = self.world.floor(point) {
            match floor {
                Floor::Nothing | Floor::ShortGrass | Floor::FruitBush(_) => {}
            }
        }

        if let Some(structure) = self.world.structure(point) {
            match structure {
                Structure::BigLeafTree(_)
                | Structure::FruitTree(_, _)
                | Structure::Campfire
                | Structure::Storage => {
                    return Err(TryBuildError::StructureAlreadyExist(structure.clone()))
                }

                Structure::Nothing => {}
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TryBuildError {
    OutsideWorld,
    IncompatibleGround(Ground),
    StructureAlreadyExist(Structure),
}
