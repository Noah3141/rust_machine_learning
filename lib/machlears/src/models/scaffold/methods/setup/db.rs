use std::path::PathBuf;

use sqlx::AnyConnection;


pub enum DatabaseOpt {
    /// MySQL, Sqlite, Postgres
    ConnectionURI(String),
    /// Provide a path to write to
    CsvFile(PathBuf),
    /// Provide a path to write to
    JsonFile(PathBuf),
    /// If No `DatabaseOpt` is chosen, iterations will not be saved 
    None
}

//  |
//  V

pub enum Database {
    Db(AnyConnection),
    CsvFile(csv::Writer<std::fs::File>),
    JsonFile(std::fs::File),
    None
}