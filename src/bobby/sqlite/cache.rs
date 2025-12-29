//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use super::database::Database;
use super::row::Row;


const CACHE_SIZE: usize = 256;
const CACHE_PAGE_SIZE: usize = 64;

#[derive(Debug)]
pub struct Cache {
    pub rows: Vec<Row>,
}


impl Database {
    pub fn cache_update(&self, _start_index: u64) -> Result<(), Box<dyn Error>> {
        todo!()
    }


    pub fn cache_get(&self, _index: u64) -> Option<Row> {
        todo!()
    }
}
