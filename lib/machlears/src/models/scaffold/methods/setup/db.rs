use std::path::PathBuf;

pub enum DatabaseOpt {
    /// Tuple (username, password)
    Sqlite(ConnectOpts),
    /// Include 
    ConnectionURI(String),
    /// Provide a path to write to
    CsvFile(PathBuf),
    /// Provide a path to write to
    JsonFile(PathBuf),
    /// If No `DatabaseOpt` is chosen, iterations will not be saved 
    None
}

pub struct ConnectOpts {
    username: String,
    password: String,
    ip: String,
    port: String,
}