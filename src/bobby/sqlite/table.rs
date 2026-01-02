//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::str;

use super::database::Database;


#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    name: String,
    has_row_id: Option<bool>,
}

impl Table {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn has_row_id(&self) -> Option<bool> {
        self.has_row_id
    }
}


// TODO: Replace with TableName(String)
impl str::FromStr for Table {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.starts_with("sqlite_") {
            return Err("Table name is empty or starts with sqlite_".into());
        }

        if !s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("Table name contains invalid characters".into());
        }

        Ok(Table { name: s.to_string(), has_row_id: None, })
    }
}


impl Database {
    pub fn tables(&self) -> Result<Vec<Table>, Box<dyn Error>> {
        let mut sql = self.connection.prepare(
            "SELECT name,
               CASE
                 WHEN type = 'table' AND sql LIKE '%WITHOUT ROWID%' THEN 0
                 WHEN type = 'table' THEN 1
                 ELSE NULL
               END AS has_row_id,
               type
             FROM sqlite_master
             WHERE type IN ('table', 'view')
               AND name NOT LIKE 'sqlite_%'
             ORDER BY name;"
        )?;

        let tables = sql.query_map([],
            |row| {
                let name: String = row.get(0)?;
                let has_row_id: Option<i64> = row.get(1)?;
                let _type: String = row.get(2)?;

                Ok(Table {
                    name,
                    has_row_id: has_row_id.map(|v| v != 0),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tables)
    }


    pub fn row_count(&self, table: &Table) -> Result<i64, Box<dyn Error>> {
        let sql = format!("SELECT COUNT(*) FROM {}", table.name());
        Ok(self.connection.query_row(&sql, [], |row| row.get(0))?)
    }
}
