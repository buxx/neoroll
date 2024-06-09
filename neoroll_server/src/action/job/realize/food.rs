use neoroll_world::{
    entity::{creature::Creature, floor::Floor, structure::Structure},
    gameplay::behavior::Behavior,
};

use crate::{
    action::{collect::CollectBuilder, move_::MoveRandomlyBuilder, ActionChange, ActionId},
    state::{State, StateChange},
};

pub struct RealizeSearchFood<'a> {
    creature: &'a Creature,
    state: &'a State,
}

impl<'a> RealizeSearchFood<'a> {
    pub fn new(creature: &'a Creature, state: &'a State) -> Self {
        Self { creature, state }
    }

    fn food_to_collect_on_place(&self) -> bool {
        let world = self.state.world();

        if let Some(Structure::FruitTree(filled)) = world.structure(self.creature.point()) {
            return !filled.is_empty();
        }

        if let Some(Floor::FruitBush(filled)) = world.floor(self.creature.point()) {
            return !filled.is_empty();
        }

        false
    }

    fn already_collecting(&self) -> bool {
        matches!(self.creature.behavior(), Behavior::Collect(_))
    }

    pub fn changes(&self) -> Vec<StateChange> {
        if self.food_to_collect_on_place() && !self.already_collecting() {
            let action_id = ActionId::new();
            let action = CollectBuilder::new(*self.creature.id()).build();
            return vec![StateChange::Action(action_id, ActionChange::New(action))];
        }

        if !self.already_collecting() {
            let action_id = ActionId::new();
            let action = MoveRandomlyBuilder::new(*self.creature.id()).build();
            return vec![StateChange::Action(action_id, ActionChange::New(action))];
        }

        vec![]
    }
}
