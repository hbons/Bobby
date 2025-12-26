//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::str;

use super::database::Database;


#[derive(Clone, Debug)]
pub struct Table(String);


impl Table {
    pub fn name(&self) -> &str {
        &self.0
    }


    pub fn row_count() -> Result<u64, Box<dyn Error>> {
        todo!()
    }
}


impl str::FromStr for Table {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.starts_with("sqlite_") {
            return Err("Table name is empty or starts with sqlite_".into());
        }

        if !s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("Table name contains invalid characters".into());
        }

        Ok(Table(s.to_string()))
    }
}


impl Database {
    pub fn tables(&self) -> Result<Vec<Table>, Box<dyn Error>> {
        let mut sql = self.connection.prepare(
            "SELECT name
             FROM sqlite_master
             WHERE type IN ('table', 'view')
               AND name NOT LIKE 'sqlite_%'
             ORDER BY name;"
        )?;

        let iter = sql.query_map([], |row| row.get::<_, String>(0))?;
        let mut tables = vec![];

        for table_name in iter {
            tables.push(table_name?.parse::<Table>()?);
        }

        Ok(tables)
    }
}
