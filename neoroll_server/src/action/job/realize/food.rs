use neoroll_world::{
    entity::creature::Creature,
    gameplay::{behavior::Behavior, CollectType},
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

        let can_from_structure = world
            .structure(self.creature.point())
            .as_ref()
            .and_then(|s| s.collectable(CollectType::Food).map(|f| !f.is_empty()))
            .unwrap_or(false);
        let can_from_floor = world
            .floor(self.creature.point())
            .and_then(|s| s.collectable(CollectType::Food).map(|f| !f.is_empty()))
            .unwrap_or(false);

        can_from_floor | can_from_structure
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
