//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use std::error::Error;

use rusqlite::types::ValueRef;

use super::column::ColumnSeparator;
use super::database::Database;
use super::table::Table;


#[derive(Clone, Debug, Default)]
pub struct Row {
    pub cells: Vec<String>,
}


const BLOB_PREVIEW_LEN: usize = 8;

impl Database {
    pub fn rows(&self, table: &Table, row_order: Option<RowOrder>) -> Result<Vec<Row>, Box<dyn Error>> {
        let sql =
            if table.has_row_id() == Some(true) {
                if let Some(order) = row_order {
                    // TODO: "LIMIT 100 OFFSET 0"
                    &format!("SELECT * FROM {} ORDER BY rowid {order};", table.name())
                } else {
                    &format!("SELECT * FROM {} ORDER BY rowid DESC;", table.name())
                }
            } else {
                &format!("SELECT * FROM {}", table.name())
            };

        let mut sql = self.connection.prepare(sql)?;
        let n_columns = sql.column_count();

        let iter = sql.query_map([], |row| {
            let mut values = Vec::new();

            for i in 0..n_columns {
                let value = match row.get_ref(i)? {
                    ValueRef::Null       => "NULL".to_string(),
                    ValueRef::Integer(i) => i.to_string(),
                    ValueRef::Real(f)    => f.to_string(),
                    ValueRef::Text(t)    => String::from_utf8_lossy(t).into(),
                    ValueRef::Blob(b)    => format!(
                        "{} BYTES:{}",
                        b.len(),
                        hex_preview(b, BLOB_PREVIEW_LEN)
                    ),
                };

                values.push(value);
            }

            Ok(values)
        })?;

        Ok(iter
            .map(|res| res.map(|cells| Row { cells }))
            .collect::<Result<Vec<_>, _>>()?
        )
    }
}

pub fn hex_preview(blob: &[u8], length: usize) -> String {
    blob
        .iter()
        .take(length)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}


impl Row {
    pub fn to_string(&self, separator: Option<ColumnSeparator>) -> String {
        match separator {
            Some(ColumnSeparator::Tabs)     => self.cells.join("\t"),
            Some(ColumnSeparator::Spaces)   => self.cells.join(" "),
            Some(ColumnSeparator::Commas)   => self.cells.join(","),
            Some(ColumnSeparator::Markdown) => format!("| {} |", self.cells.join(" | ")),
            None => String::new(),
        }
    }
}


#[derive(Debug, Default)]
pub enum RowOrder {
    #[default]
    Descending,
    Ascending,
}

impl fmt::Display for RowOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RowOrder::Descending => "DESC",
            RowOrder::Ascending  => "ASC",
        };
        write!(f, "{}", s)
    }
}
