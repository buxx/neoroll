use std::sync::RwLockReadGuard;

use neoroll_world::{
    entity::{creature::Creature, structure::Structure},
    gameplay::{
        behavior::Behavior,
        material::{Material, Resource},
        tribe::{structure::StructureOwn, TribeId},
        CollectType, Quantity,
    },
};

use crate::{
    action::{
        collect::CollectBuilder, drop::DropOff, move_::MoveRandomlyBuilder, Action, ActionChange,
        ActionId,
    },
    state::{game::GameState, State, StateChange},
};

const FOOD_LIMIT_QUANTITY: Quantity = Quantity(4000);

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

    fn already_dropping_off(&self) -> bool {
        matches!(self.creature.behavior(), Behavior::DropOff)
    }

    pub fn nearest_storages(
        &'a self,
        tribe_id: &TribeId,
        game: &'a RwLockReadGuard<GameState>,
    ) -> Vec<&StructureOwn> {
        // FIXME BS NOW: order by distance
        game.tribe_structures(tribe_id, Some(Structure::Storage))
    }

    pub fn changes(&self) -> Vec<StateChange> {
        let carrying_food = self
            .creature
            .carrying_quantity(Some(Material::Resource(Resource::Food)))
            .0
            > 0;
        let already_collecting = self.already_collecting();
        let already_dropping_off = self.already_dropping_off();
        let carrying_too_much_food =
            self.creature.carrying_quantity(None).0 >= FOOD_LIMIT_QUANTITY.0;

        if !already_dropping_off && carrying_food && carrying_too_much_food {
            let tribe_id = self.creature.tribe_id();
            let game = self.state.game();
            if let Some(storage) = self.nearest_storages(tribe_id, &game).first() {
                let action_id = ActionId::new();
                let action = Action::DropOff(DropOff::new(
                    *self.creature.id(),
                    *storage.point(),
                    Material::Resource(Resource::Food),
                ));
                return vec![StateChange::Action(action_id, ActionChange::New(action))];
            }
        }

        if self.food_to_collect_on_place() && !already_collecting && !already_dropping_off {
            let action_id = ActionId::new();
            let action = CollectBuilder::new(*self.creature.id()).build();
            return vec![StateChange::Action(action_id, ActionChange::New(action))];
        }

        if !already_collecting && !already_dropping_off {
            let action_id = ActionId::new();
            let action = MoveRandomlyBuilder::new(*self.creature.id()).build();
            return vec![StateChange::Action(action_id, ActionChange::New(action))];
        }

        vec![]
    }
}
