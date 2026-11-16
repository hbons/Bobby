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


#[derive(Clone, Debug, Default)]
pub struct Column {
    pub id: u64,
    pub name: ColumnName,
    pub affinity: Affinity,
    pub primary_key: bool,
    pub not_null: bool,
    pub default: Option<String>,
    pub unique: bool,
}


impl Database {
    pub fn columns(&self, table: &Table) -> Result<Vec<Column>, Box<dyn Error>> {
        let connection = self.connection.borrow();

        let mut sql = connection.prepare(
            &format!("PRAGMA table_info({});", table.name.quoted())
        )?;

        let iter = sql.query_map([], |row| {
            let cell1: String = row.get(1)?;
            let cell2: String = row.get(2)?;

            let name = cell1
                .parse::<ColumnName>()
                .unwrap_or_default();

            let affinity = cell2
                .parse::<Affinity>()
                .unwrap_or_default();

            Ok(Column {
                id: row.get(0)?,
                name,
                affinity,
                not_null: row.get(3)?,
                default: row.get(4)?,
                primary_key: row.get(5)?,
                unique: false, // TODO
            })
        })?;

        Ok(iter.collect::<Result<Vec<_>, _>>()?)
    }
}


pub fn as_filter_statement(
    columns: &[Column],
    filter: String,
) -> String
{
    let mut sql = vec![];

    // TODO: Convert non-ascii numbers to ascii
    let filter_not_numeric = filter
        .parse::<f64>()
        .is_err();

    for column in columns {
        let part = format!("{} LIKE '%{}%'",
            column.name.quoted(),
            filter
        );

        match column.affinity {
            Affinity::NULL => continue,
            Affinity::BLOB(_, _) => continue,
            Affinity::INTEGER(_) |
            Affinity::NUMERIC(_) |
            Affinity::REAL(_) => {
                if filter_not_numeric {
                    continue;
                }

                sql.push(part);
            },
            Affinity::TEXT(_) => sql.push(part),
        };
    }

    sql.join(" OR ")
}


#[derive(Debug, Default)]
pub enum ColumnSeparator {
    #[default] Tabs,
    Spaces,
    Commas,
    Markdown,
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


#[derive(Clone, Debug, Default, PartialEq)]
pub struct ColumnName(String);

impl str::FromStr for ColumnName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Column name is empty".into());
        }

        Ok(Self(s.to_string()))
    }
}

impl ColumnName {
    /// For use in queries
    pub fn quoted(&self) -> String {
        let s = self.0.replace("\"", "\"\"");
        format!("\"{s}\"")
    }

    /// For use in user interface controls
    pub fn unquoted(&self) -> String {
        self.0.clone()
    }
}
