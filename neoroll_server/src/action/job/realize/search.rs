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
    map::find::AroundTileFinder,
    space::AbsoluteWorldPoint,
};

use crate::{
    action::{
        collect::CollectBuilder, drop::DropOff, move_::MoveTo, Action, ActionChange, ActionId,
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

    fn moving_to(&self) -> bool {
        matches!(self.creature.behavior(), Behavior::MoveTo)
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

    pub fn find_collect_tile_point(&self) -> Option<AbsoluteWorldPoint> {
        AroundTileFinder::new(&self.state.world(), *self.creature.point())
            .collect(Some(self.resource.into()))
            .search()
    }

    pub fn changes(&self) -> Vec<StateChange> {
        let _solving = self.solving();
        let carrying = self.carrying();
        let collecting = self.collecting();
        let moving_to = self.moving_to();
        let dropping_off = self.dropping_off();
        let carrying_enough = self.carrying_enough();

        if !dropping_off && carrying && carrying_enough && !moving_to {
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

        if self.can_collect() && !collecting && !dropping_off && !moving_to {
            let action_id = ActionId::new();
            let action = CollectBuilder::new(*self.creature.id(), self.resource).build();
            return vec![StateChange::Action(action_id, ActionChange::New(action))];
        }

        // FIXME BS NOW DEV: AroundTileFinder must be configured to ensure (and return) a walkable path to the result
        if !collecting && !dropping_off && !moving_to {
            if let Some(point) = self.find_collect_tile_point() {
                let action_id = ActionId::new();
                let action = Action::MoveTo(MoveTo::new(*self.creature.id(), point));
                return vec![StateChange::Action(action_id, ActionChange::New(action))];
            }
        }

        vec![]
    }
}
