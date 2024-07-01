use bevy_egui::egui::{include_image, ImageSource};

pub enum Illustration {
    ArtocarpusAltilis
}

impl Illustration {
    pub fn data(&self) -> ImageSource {
        match self {
            Illustration::ArtocarpusAltilis => include_image!("../../data/images/Artocarpus_altilis.png"),
        }
    }
}