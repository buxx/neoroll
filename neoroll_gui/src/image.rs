use bevy_egui::egui::{include_image, ImageSource};

pub enum Illustration {
    ArtocarpusAltilis,
    Soil,
    ShortAndDryGrass,
    ShortAndDryGrassButton,
    AbuluntuEmpetrumNigrum,
    FlintQuary,
    ApricotTree,
    Campfire,
    CampfireButton,
    Fruits1,
    CurvedFlint1,
    FreshWater1,
    RawFlint1,
}

impl Illustration {
    pub fn data(&self) -> ImageSource {
        match self {
            Illustration::ArtocarpusAltilis => {
                include_image!("../../data/images/Artocarpus_altilis.png")
            }
            Illustration::Soil => include_image!("../../data/images/soil1.png"),
            Illustration::ShortAndDryGrass => {
                include_image!("../../data/images/Short_dry_grass.png")
            }
            Illustration::ShortAndDryGrassButton => {
                include_image!("../../data/images/Short_dry_grass_button.png")
            }
            Illustration::AbuluntuEmpetrumNigrum => {
                include_image!("../../data/images/Abuluntu_Empetrum_nigrum.png")
            }
            Illustration::FlintQuary => include_image!("../../data/images/Flint_Quary.png"),
            Illustration::ApricotTree => include_image!("../../data/images/ApricotTree.png"),
            Illustration::Campfire => include_image!("../../data/images/Campfire.png"),
            Illustration::CampfireButton => include_image!("../../data/images/Campfire_button.png"),
            Illustration::Fruits1 => include_image!("../../data/images/Fruits1.png"),
            Illustration::CurvedFlint1 => include_image!("../../data/images/CurvedFlint1.png"),
            Illustration::FreshWater1 => include_image!("../../data/images/FreshWater1.png"),
            Illustration::RawFlint1 => include_image!("../../data/images/RawFlint1.png"),
        }
    }
}
