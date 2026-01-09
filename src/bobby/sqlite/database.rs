//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::{ Path, PathBuf };
use std::time::Duration;

use rusqlite::{ Connection, OpenFlags };


#[derive(Debug)]
pub struct Database {
    pub path: PathBuf,
    pub connection: Connection,
}


/// Database files to test on can be found at:
/// http://2016.padjo.org/tutorials/sqlite-data-starterpacks
impl Database {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY
        )?;

        connection.busy_timeout(Duration::from_secs(3))?;
        connection.execute_batch("PRAGMA query_only = ON;")?;
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(
            Database {
                path: path.to_path_buf(),
                connection,
            }
        )
    }


    pub fn data_version(&self) -> Option<i64> {
        let version: i64 = self.connection.query_row(
            "PRAGMA data_version;",
            [],
            |row| row.get(0),
        ).ok()?;

        Some(version)
    }
}
