//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::path::{ Path, PathBuf };
use std::error::Error;

use rusqlite::Connection;


#[derive(Debug)]
pub struct Database {
    pub path: PathBuf,
    pub connection: Connection,
}


impl Database {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let connection = Connection::open(path)?;

        Ok(
            Database {
                path: path.to_path_buf(),
                connection,
            }
        )
    }
}
