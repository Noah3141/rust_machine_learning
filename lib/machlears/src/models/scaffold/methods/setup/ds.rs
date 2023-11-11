use std::path::PathBuf;
use crate::prelude::DatabaseOpt;


pub enum DataSourceOpt {
    /// Provide a path to write to
    CsvFile(PathBuf),
    /// Provide a path to write to
    ExcelFile(PathBuf),
    /// JSON data can be provided as either 
    /// 
    /// `1)` with a top level object and "data" key:
    /// ```
    /// {
    ///     "data": [
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///     ]
    /// }
    /// ```
    /// or 
    /// 
    /// `2)` with a top level array:
    /// ```
    /// [
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    /// 
    /// ]
    /// ```
    JsonFile(PathBuf),
    /// JSON data can be provided as either 
    /// 
    /// `1)` with a top level object and "data" key:
    /// ```
    /// {
    ///     "data": [
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///     ]
    /// }
    /// ```
    /// or 
    /// 
    /// `2)` with a top level array:
    /// ```
    /// [
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    ///         {
    ///             "header_1": "value"
    ///             "header_2": "value"
    ///         },
    /// 
    /// ]
    /// ```
    JsonData(serde_json::Value),
}

// |
// V

pub enum DataSource {
    CsvFile(std::fs::File),
    ExcelFile(std::fs::File),
    JsonFile(std::fs::File),
    JsonData(serde_json::Value),
}