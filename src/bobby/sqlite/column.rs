//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::str;

use super::affinity::Affinity;
use super::database::Database;
use super::table::Table;


#[derive(Clone, Debug)]
pub struct Column {
    pub id: u64,
    pub name: String,
    pub affinity: Affinity,
    pub primary_key: bool,
    pub not_null: bool,
    pub default: Option<String>,
}


impl Database {
    pub fn columns(&self, table: &Table) -> Result<Vec<Column>, Box<dyn Error>> {
        let mut sql = self.connection.prepare(
            &format!("PRAGMA table_info({});", table.name())
        )?;

        let iter = sql.query_map([], |row| {
            let cell2: String = row.get(2)?;

            let affinity = cell2
                .parse::<Affinity>()
                .unwrap_or_default();

            Ok(Column {
                id:          row.get(0)?,
                name:        row.get(1)?,
                affinity,
                not_null:    row.get(3)?,
                default:     row.get(4)?,
                primary_key: row.get(5)?,
            })
        })?;

        Ok(iter.collect::<Result<Vec<_>, _>>()?)
    }
}


#[derive(Debug, Default)]
pub enum ColumnSeparator {
    #[default]
    Tabs,
    Spaces,
    Commas,
    Markdown
}

impl str::FromStr for ColumnSeparator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tabs"   | "\t" => Ok(Self::Tabs),
            "spaces" | " "  => Ok(Self::Spaces),
            "commas" | ","  => Ok(Self::Commas),
            "markdown"      => Ok(Self::Markdown),
            _ => Err("Invalid column separator".into())
        }
    }
}
