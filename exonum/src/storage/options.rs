//! Abstract settings for databases.

/// Options for the database.
///
/// These parameters apply to the underlying database of Exonum, currently `RocksDB`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DbOptions {
    /// Number of open files that can be used by the database.
    ///
    /// The underlying database opens multiple files during operation. If your system has a
    /// limit on the number of files which can be open simultaneously, you can
    /// adjust this option to match the limit. Note, that limiting the number
    /// of simultaneously open files might slow down the speed of database operation.
    ///
    /// Defaults to `None`, meaning that the number of open files is unlimited.
    pub max_open_files: Option<i32>,
    /// An option to indicate whether the system should create a database or not,
    /// if it's missing.
    ///
    /// This option applies to the cases when a node was
    /// switched off and is on again. If the database cannot be found at the
    /// indicated path and this option is switched on, a new database will be
    /// created at that path and blocks will be included therein.
    ///
    /// Defaults to `true`.
    pub create_if_missing: bool,
}

impl Default for DbOptions {
    fn default() -> Self {
        Self {
            max_open_files: None,
            create_if_missing: true,
        }
    }
}
