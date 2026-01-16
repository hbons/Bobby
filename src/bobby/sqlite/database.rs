//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::cell::RefCell;
use std::error::Error;
use std::path::{ Path, PathBuf };
use std::rc::Rc;
use std::time::Duration;

use rusqlite::{
    Connection,
    OpenFlags
};

use super::row::RowOrder;


#[derive(Debug)]
pub struct Database {
    pub path: PathBuf,
    pub connection: Rc<RefCell<Connection>>,
    pub row_order: Option<RowOrder>,
}


/// Database files to test on can be found at:
/// http://2016.padjo.org/tutorials/sqlite-data-starterpacks
impl Database {
    pub fn from_file(path: &Path, row_order: Option<RowOrder>) -> Result<Self, Box<dyn Error>> {
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
                connection: Rc::new(RefCell::new(connection)),
                row_order,
            }
        )
    }


    pub fn data_version(&self) -> Option<i64> {
        let connection = self.connection.borrow();

        let version: i64 = connection.query_row(
            "PRAGMA data_version;",
            [],
            |row| row.get(0),
        ).ok()?;

        Some(version)
    }
}


impl Default for Database {
    #[allow(clippy::expect_used)]
    fn default() -> Self {
        let default_path = PathBuf::from(":memory:");
        let connection = Connection::open_in_memory()
            // This should never happen
            .expect("Failed to create default connection");

        Database {
            path: default_path,
            connection: Rc::new(RefCell::new(connection)),
            row_order: None,
        }
    }
}


impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            connection: Rc::clone(&self.connection),
            row_order: self.row_order.clone(),
        }
    }
}
