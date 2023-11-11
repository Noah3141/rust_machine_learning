use crate::models::info::entity::Info;
use super::methods::setup::*;



pub struct MachScaffold<'a> {
    database: db::DatabaseOpt,
    datasource: ds::DataSource,
    print_style: ps::PrintStyle<'a, dyn Fn(&Info) -> ()>
}
