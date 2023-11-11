use crate::models::{scaffold, info::entity::Info};

pub struct MachTrainer<'func, Function> where Function: Fn(&Info) -> () {
    pub(crate) database: scaffold::methods::setup::db::Database,
    pub(crate) datasource: scaffold::methods::setup::ds::DataSource,
    pub(crate) print_style: scaffold::methods::setup::ps::PrintStyle<'func, Function>
}

