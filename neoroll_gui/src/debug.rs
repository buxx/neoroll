use std::collections::HashMap;

use neoroll_world::{
    entity::ground::Ground,
    space::{world::World, AbsoluteWorldColI, AbsoluteWorldPoint, AbsoluteWorldRowI},
};

pub struct WorldToTxt<'a> {
    world: &'a World,
    default: char,
    grounds: HashMap<Ground, char>,
}

impl<'a> WorldToTxt<'a> {
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            default: '0',
            grounds: HashMap::new(),
        }
    }

    pub fn default(mut self, value: char) -> Self {
        self.default = value;
        self
    }

    pub fn ground(mut self, ground: Ground, value: char) -> Self {
        self.grounds.insert(ground, value);
        self
    }

    pub fn build(&self) -> String {
        let mut content = "".to_string();

        for row_i in 0..self.world.lines() {
            for col_i in 0..self.world.columns() {
                let ground = self.world.ground(&AbsoluteWorldPoint(
                    AbsoluteWorldRowI(row_i as isize),
                    AbsoluteWorldColI(col_i as isize),
                ));
                let char = self
                    .grounds
                    .get(ground.expect("Must have ground here"))
                    .unwrap_or(&self.default);
                content.push_str(&char.to_string())
            }
            content.push('\n');
        }

        content
    }
}
