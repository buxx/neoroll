use crate::{
    entity::{floor::Floor, ground::Ground},
    space::{
        layer::{CompositeLayer, FilledLayer, Layers},
        world::World,
    },
};

pub struct WorldFromStrBuilder<'a> {
    raw: &'a str,
}

impl<'a> WorldFromStrBuilder<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self { raw }
    }

    pub fn build(&self) -> World {
        let lines = self.raw.lines().collect::<Vec<&str>>();
        let columns = lines.first().unwrap_or(&"").len();
        let mut grounds = vec![];

        for line in &lines {
            for char in line.trim().chars() {
                if char == '1' {
                    grounds.push(Ground::FreshWater)
                } else {
                    grounds.push(Ground::Soil)
                }
            }
        }

        World::new(
            Layers::new(
                FilledLayer::new(grounds),
                FilledLayer::new(vec![Floor::Nothing; lines.len() * columns]),
                CompositeLayer::new(vec![None; lines.len() * columns]),
                FilledLayer::new(vec![vec![]; lines.len() * columns]),
            ),
            lines.len(),
            columns,
            vec![],
        )
    }
}
