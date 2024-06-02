use bevy::ecs::event::{Event, EventReader};

pub trait EventReaderShortcuts {
    fn has_been_set(&mut self) -> bool;
}

impl<'w, 's, E: Event> EventReaderShortcuts for EventReader<'w, 's, E> {
    fn has_been_set(&mut self) -> bool {
        self.iter().collect::<Vec<&E>>().last().is_some()
    }
}
