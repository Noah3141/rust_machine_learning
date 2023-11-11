use sqlx::{ConnectOptions};

use crate::models::{trainer::entity::MachTrainer, active::entity::Mach, info::entity::Info};

use self::ps::PrintStyle;

pub mod db;
pub mod ds;
pub mod ps;

impl super::super::entity::MachScaffold<'_> {

    /// Provide the initial information to begin setting up a model. Takes in [`Opts`]
    pub async fn setup<'a, Func>(self, opts: crate::models::scaffold::methods::setup::Opts<'a, Func>) -> MachTrainer<Func> where Func: Fn(&Info) {
        let db = self.handle_database_opt(opts.database).await;
        let ds = self.handle_datasource_opt(opts.datasource);
        let ps = self.handle_print_style_opt(opts.print_style);
        
        MachTrainer {
            database: db,
            datasource: ds,
            print_style: ps,

        }
    }


    fn handle_datasource_opt(&self, ds_opt: ds::DataSourceOpt) -> ds::DataSource {
        match ds_opt {
            ds::DataSource::CsvFile(_) => todo!(),
            ds::DataSource::ExcelFile(_) => todo!(),
            ds::DataSource::JsonFile(_) => todo!(),
            ds::DataSource::JsonData(_) => todo!(),
        }
    }

    async fn handle_database_opt(&self, db_opt: db::DatabaseOpt) -> db::Database {
        sqlx::any::install_default_drivers();
        match db_opt {
            db::DatabaseOpt::ConnectionURI(uri) => {
                let url = &url::Url::parse(&uri).expect("URI valid");
                let db = sqlx::any::AnyConnectOptions::from_url(url)
                    .expect("Connection options")
                    .connect()
                    .await
                    .expect("Connection");
                db::Database::Db(db)
            },
            db::DatabaseOpt::CsvFile(filepath) => {
                let file = std::fs::File::options().create(true).read(true).append(true).open(filepath).expect("To create or open file at provided path");
                let mut writer = csv::Writer::from_writer(file);
                writer.serialize(Info::test_write()).expect("Test write failed");
                db::Database::CsvFile(writer)
            },
            db::DatabaseOpt::JsonFile(filepath) => {
                let file = std::fs::File::options().create(true).read(true).append(true).open(filepath).expect("To create or open file at provided path");
                let mut writer = serde_json::to_writer_pretty(
                    &file, 
                    &serde_json::to_value(Info::test_write()).expect("serialization")
                    ).expect("Test write failed");
                db::Database::JsonFile(file)
            },
            db::DatabaseOpt::None => db::Database::None,
        }
    }

    fn handle_print_style_opt<'a, Function>(&self, ps_opt: ps::PrintStyle<'a, Function>) -> PrintStyle<'a, Function> where Function: Fn(&Info) -> () + ?Sized {
        match ps_opt {
            ps::PrintStyle::Silent => todo!(),
            ps::PrintStyle::Default => todo!(),
            ps::PrintStyle::All => todo!(),
            ps::PrintStyle::Custom(f) => todo!(),
        }
    }


    
}

pub struct Opts<'a, Func> where Func: Fn(&Info) + ?Sized {
    /// Choose a data input source. Provide a filepath for files. 
    pub datasource: ds::DataSourceOpt,
    /// Choose a database connection option. This allows iterations of testing to be stored for comparison. 
    pub database: db::DatabaseOpt,
    /// Choose what aspects to print during training. [`ps::PrintStyle`]
    pub print_style: ps::PrintStyle<'a, Func>
}
