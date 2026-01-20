//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::str;

use super::database::Database;


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    name: TableName,
    has_row_id: Option<bool>,
    is_view: bool,
}

impl Table {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn has_row_id(&self) -> Option<bool> {
        self.has_row_id
    }

    pub fn is_view(&self) -> bool {
        self.is_view
    }
}


impl Database {
    pub fn tables(&self) -> Result<Vec<Table>, Box<dyn Error>> {
        let connection = self.connection.borrow();

        let mut sql = connection.prepare(
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
                let type_str: String = row.get(2)?;

                let name = name
                    .parse::<TableName>()
                    .map_err(|_| rusqlite::Error::InvalidQuery)?;

                Ok(Table {
                    name,
                    has_row_id: has_row_id.map(|v| v != 0),
                    is_view: type_str == "view",
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tables)
    }


    pub fn row_count(&self, table: &Table) -> Result<u32, Box<dyn Error>> {
        let connection = self.connection.borrow();

        let sql = format!("SELECT COUNT(*) FROM {}", table.name());
        Ok(connection.query_row(&sql, [], |row| row.get(0))?)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct TableName(String);

impl str::FromStr for TableName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.starts_with("sqlite_") {
            return Err("Table name is empty or starts with sqlite_".into());
        }

        if !s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("Table name contains invalid characters".into());
        }

        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for TableName {
    fn default() -> Self {
        Self("table".into())
    }
}
