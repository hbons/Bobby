//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::str;

use rusqlite::params;

use super::column::{ self, Column };
use super::database::Database;
use super::row::Row;


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    pub name: TableName,
    /// Views don't support UPDATE or DELETE
    pub is_view: bool,
    /// On views this is None
    pub has_row_id: Option<bool>,
    /// If created with WITHOUT_ROWID, there must be a PRIMARY_KEY
    pub primary_key: Option<Column>,
    pub filter: Option<String>,
}


pub struct View {
    pub name: TableName,
    pub filter: Option<String>,
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
                let name = name
                    .parse::<TableName>()
                    .map_err(|_| rusqlite::Error::InvalidQuery)?;

                let has_row_id: Option<i64> = row.get(1)?;
                let has_row_id = has_row_id.map(|v| v == 1);

                let is_view: String = row.get(2)?;
                let is_view = is_view == "view";

                Ok(
                    Table {
                        name,
                        is_view,
                        has_row_id,
                        primary_key: None, // TODO
                        filter: None,
                    }
                )
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tables)
    }
}


impl Database {
    pub fn row_count(&self, table: &Table) -> Result<u32, Box<dyn Error>> {
        let conn = self.connection.borrow();

        let filter = match table.filter {
            Some(ref f) =>
                column::as_filter_statement(
                    &self.columns(table).unwrap_or_default(),
                    f.into(),
                ),
            None => "TRUE".to_string(),
        };

        let sql = format!(
            "SELECT COUNT(*) FROM {table} WHERE {filter}",
            table=table.name,
        );

        Ok(conn.query_row(&sql, [], |row| row.get(0))?)
    }


    pub fn row_count_no_filter(&self, table: &Table) -> Result<u32, Box<dyn Error>> {
        let connection = self.connection.borrow();
        let sql = format!("SELECT COUNT(*) FROM {}", table.name);

        Ok(connection.query_row(&sql, [], |row| row.get(0))?)
    }
}


impl Database {
    pub fn update(
        &self,
        table: &Table,
        column: &Column,
        row: &Row,
        value: &str,
    ) -> Result<(), Box<dyn Error>>
    {
        let primary_key = self.primary_key(&table)?; // TODO: ok_or("ROWID")

        self.connection
            .borrow()
            .execute(
                &format!("UPDATE {} SET {} = ?1 WHERE {} = ?2;",
                    table.name.to_string(),
                    column.name,
                    primary_key,
                ),
                params![value, row.id]
            )?; // Can fail on tables created with STRICT typing

        Ok(())
    }


    pub fn delete(
        &self,
        table: &Table,
        row: &Row,
    ) -> Result<(), Box<dyn Error>>
    {
        let primary_key = self.primary_key(&table)?; // TODO: ok_or("ROWID")

        self.connection
            .borrow()
            .execute(
                &format!("DELETE FROM {} WHERE {} = ?1;",
                    table.name.to_string(),
                    table.primary_key.ok_or("Missing PRIMARY_KEY")?,
                ),
                params![row.id]
            )?;

        Ok(())
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct TableName(String);

impl str::FromStr for TableName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.to_ascii_lowercase().starts_with("sqlite_") {
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
