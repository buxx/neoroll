use crate::{entity::structure::Structure, gameplay::tribe::TribeId, space::AbsoluteWorldPoint};

#[derive(Debug, Clone)]
pub struct StructureOwn {
    type_: Structure,
    tribe_id: TribeId,
    point: AbsoluteWorldPoint,
}

impl StructureOwn {
    pub fn new(type_: Structure, tribe_id: TribeId, point: AbsoluteWorldPoint) -> Self {
        Self {
            type_,
            tribe_id,
            point,
        }
    }

    pub fn type_(&self) -> &Structure {
        &self.type_
    }

    pub fn tribe_id(&self) -> &TribeId {
        &self.tribe_id
    }

    pub fn point(&self) -> &AbsoluteWorldPoint {
        &self.point
    }
}
