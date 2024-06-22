use super::{job::Job, material::Material, Quantity};

#[derive(Debug, Clone, PartialEq)]
pub enum Need {
    MaterialInStorages(Material, Quantity),
}

// FIXME BS NOW: use it in actin to avoid duplicate code
impl From<&Need> for Job {
    fn from(value: &Need) -> Self {
        match value {
            Need::MaterialInStorages(material, _) => {
                //
                match material {
                    Material::Resource(resource) => Job::SearchResource(*resource),
                }
            }
        }
    }
}
