use std::sync::RwLockReadGuard;

use neoroll_world::{
    entity::{creature::Creature, structure::Structure},
    gameplay::{
        behavior::Behavior,
        job::Job,
        material::{Material, Resource},
        target::Target,
        tribe::{structure::StructureOwn, TribeId},
    },
};

use crate::{
    action::{
        collect::CollectBuilder, drop::DropOff, move_::MoveRandomlyBuilder, Action, ActionChange,
        ActionId,
    },
    state::{game::GameState, State, StateChange},
};

pub struct RealizeSearchResource<'a> {
    creature: &'a Creature,
    state: &'a State,
    resource: Resource,
}

impl<'a> RealizeSearchResource<'a> {
    pub fn new(creature: &'a Creature, state: &'a State, resource: Resource) -> Self {
        Self {
            creature,
            state,
            resource,
        }
    }

    fn can_collect(&self) -> bool {
        self.state
            .world()
            .can_collect(self.creature.point(), self.resource.into())
    }

    fn solving(&self) -> bool {
        for _requirement in Job::SearchResource(self.resource).requirements() {
            // FIXME: for SearchQualityWood, must own or going to pick up axe
        }

        true
    }

    fn carrying(&self) -> bool {
        self.creature
            .carrying_quantity(Some(Material::Resource(self.resource)))
            .0
            > 0
    }

    fn collecting(&self) -> bool {
        matches!(self.creature.behavior(), Behavior::Collect(_))
    }

    fn dropping_off(&self) -> bool {
        matches!(self.creature.behavior(), Behavior::DropOff)
    }

    fn carrying_enough(&self) -> bool {
        let carrying_quantity = self
            .creature
            .carrying_quantity(Some(Material::Resource(self.resource)))
            .0;
        let enough_quantity =
            Target::KeepStock(Material::Resource(self.resource), Default::default())
                .carrying_enough_quantity()
                .0;

        println!("{} >= {}", carrying_quantity, enough_quantity);
        carrying_quantity >= enough_quantity
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
        let _solving = self.solving();
        let carrying = self.carrying();
        let collecting = self.collecting();
        let dropping_off = self.dropping_off();
        let carrying_enough = self.carrying_enough();

        // dbg!((carrying, collecting, dropping_off, carrying_enough,));

        if !dropping_off && carrying && carrying_enough {
            // println!("1");
            let tribe_id = self.creature.tribe_id();
            let game = self.state.game();
            if let Some(storage) = self.nearest_storages(tribe_id, &game).first() {
                let action_id = ActionId::new();
                let action = Action::DropOff(DropOff::new(
                    *self.creature.id(),
                    *storage.point(),
                    Material::Resource(self.resource),
                ));
                return vec![StateChange::Action(action_id, ActionChange::New(action))];
            }
        }

        if self.can_collect() && !collecting && !dropping_off {
            // println!("2");
            let action_id = ActionId::new();
            let action = CollectBuilder::new(*self.creature.id(), self.resource).build();
            return vec![StateChange::Action(action_id, ActionChange::New(action))];
        }

        if !collecting && !dropping_off {
            // println!("3");
            let action_id = ActionId::new();
            let action = MoveRandomlyBuilder::new(*self.creature.id()).build();
            return vec![StateChange::Action(action_id, ActionChange::New(action))];
        }

        // println!("4");
        vec![]
    }
}