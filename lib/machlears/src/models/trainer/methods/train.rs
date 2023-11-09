use crate::models::builder::entity::MachBuilder;

impl super::super::entity::MachTrainer {
    pub fn train(&self, opts: crate::models::trainer::methods::train::Opts) -> MachBuilder {
        MachBuilder {}
    }
}

pub struct Opts {
    
}