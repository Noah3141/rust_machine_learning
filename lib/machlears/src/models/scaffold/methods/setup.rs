use crate::models::trainer::entity::MachTrainer;
pub mod db;
pub mod ds;
pub mod ps;

impl super::super::entity::MachScaffold {

    /// Provide the initial information to begin setting up a model. Takes in [`Opts`]
    pub fn setup(opts: crate::models::scaffold::methods::setup::Opts) -> MachTrainer {
        MachTrainer {}
    }

    
}

pub struct Opts {
    /// Choose a database connection option. This allows iterations of testing to be stored for comparison. 
    pub database: db::DatabaseOpt,
    /// Choose a data input source. Provide a filepath for files. 
    pub datasource: ds::DataSource,
    /// Choose what aspects to print during training. [`ps::PrintStyle`]
    pub print_style: ps::PrintStyle
}
