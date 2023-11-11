use crate::models::{active::entity::Mach, info::entity::Info};

impl<'a, Func> super::super::entity::MachTrainer<'a, Func> where Func: Fn(&Info) -> () {
    pub fn db_use_best(&self, opts: crate::models::trainer::methods::train::Opts) -> Mach {


        Mach {}
    }
}

pub struct Opts {

}