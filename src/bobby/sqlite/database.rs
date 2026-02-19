//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::cell::RefCell;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use gio::prelude::FileExt;
use gio::File;

use rusqlite::{
    Connection,
    OpenFlags,
};

use super::row::RowOrder;


#[derive(Debug)]
pub struct Database {
    pub file: File,
    pub connection: Rc<RefCell<Connection>>,
    pub row_order: Option<RowOrder>,
    pub read_only: bool,
}


/// Database files to test on can be found at:
/// http://2016.padjo.org/tutorials/sqlite-data-starterpacks
impl Database {
    pub fn from_file(file: &File, row_order: Option<RowOrder>) -> Result<Self, Box<dyn Error>> {
        let uri = file.uri();

        let (connection, read_only) = match Connection::open_with_flags(
            &uri,
            OpenFlags::SQLITE_OPEN_READ_WRITE |
            OpenFlags::SQLITE_OPEN_URI,
        ) {
            Ok(connection) => (connection, false),
            Err(_) => (
                Connection::open_with_flags(
                    &uri,
                    OpenFlags::SQLITE_OPEN_READ_ONLY |
                    OpenFlags::SQLITE_OPEN_URI,
                ).map_err(|_|
                    Box::<dyn Error>::from("Could not open a database connection")
                )?,
                true,
            )
        };

        if Database::journal_mode(&connection).is_none() {
            return Err("File is not a <b>SQLite database</b>".into());
        }

        connection.busy_timeout(Duration::from_secs(3))?;
        connection.pragma_update(None, "query_only", read_only)?;
        connection.pragma_update(None, "foreign_keys", true)?;

        Ok(
            Database {
                file: file.to_owned(),
                connection: Rc::new(RefCell::new(connection)),
                row_order,
                read_only,
            }
        )
    }


    pub fn data_version(&self) -> Option<i64> {
        let connection = self.connection.borrow();

        connection.pragma_query_value(
            None,
            "data_version",
            |row| row.get(0)
        ).ok()
    }


    pub fn journal_mode(connection: &Connection) -> Option<String> {
        let mode: String = connection
            .query_row(
                "PRAGMA journal_mode",
                [],
                |row| row.get(0)
            )
        .ok()?;

        Some(mode)
    }
}


impl Default for Database {
    #[allow(clippy::expect_used)]
    fn default() -> Self {
        let connection = Connection::open_in_memory()
            // This should never happen
            .expect("Failed to create default connection");

        Database {
            file: File::for_path(Path::new("")),
            connection: Rc::new(RefCell::new(connection)),
            row_order: None,
            read_only: true,
        }
    }
}


impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            file: self.file.clone(),
            connection: Rc::clone(&self.connection),
            row_order: self.row_order,
            read_only: self.read_only,
        }
    }
}
