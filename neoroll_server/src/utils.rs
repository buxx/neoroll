use neoroll_world::{entity::creature::Creature, gameplay::job::Job};

pub trait CreaturesJobUtils {
    fn filter_job(&self, job: &Job) -> Vec<&Creature>;
}

impl CreaturesJobUtils for Vec<&Creature> {
    fn filter_job(&self, job: &Job) -> Vec<&Creature> {
        self.into_iter()
            .filter(|c| c.job() == job)
            .map(|c| *c)
            .collect::<Vec<&Creature>>()
    }
}
