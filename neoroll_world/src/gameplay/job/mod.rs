use requirement::JobRequirement;
use serde::{Deserialize, Serialize};

use super::material::{Material, Resource};

pub mod requirement;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Job {
    Idle,
    SearchResource(Resource),
}

impl Default for Job {
    fn default() -> Self {
        Self::Idle
    }
}

impl From<&Material> for Job {
    fn from(value: &Material) -> Self {
        match value {
            Material::Resource(Resource::Food) => Job::SearchResource(Resource::Food),
            Material::Resource(Resource::RawFlint) => Job::SearchResource(Resource::RawFlint),
        }
    }
}

impl Job {
    pub fn requirements(&self) -> Vec<JobRequirement> {
        match self {
            Job::Idle => vec![],
            Job::SearchResource(resource) => match resource {
                Resource::Food => vec![],
                Resource::RawFlint => vec![],
            },
        }
    }
}
