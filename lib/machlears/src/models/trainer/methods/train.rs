use crate::models::{builder::entity::MachBuilder, info::entity::Info};

impl<'a, Func> super::super::entity::MachTrainer<'a, Func> where Func: Fn(&Info) -> () {
    pub fn train(&self, opts: crate::models::trainer::methods::train::Opts) -> MachBuilder {
        MachBuilder {}
    }
}

pub struct Opts {
    
}