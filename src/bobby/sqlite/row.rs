//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;

use rusqlite::types::ValueRef;

use super::affinity::Affinity;
use super::column::ColumnSeparator;
use super::database::Database;
use super::table::Table;


#[derive(Clone, Debug, Default)]
pub struct Row {
    pub cells: Vec<Affinity>,
}


const BLOB_PREVIEW_LEN: usize = 8;

impl Database {
    pub fn rows(
        &self,
        table: &Table,
        offset: Option<u32>,
        limit:  Option<u32>,
    ) -> Result<Vec<Row>, Box<dyn Error>>
{
        let limit = limit.unwrap_or(u32::MAX); // GTK models are limited to u32
        let offset = offset.unwrap_or(0);
        let row_order = self.row_order.unwrap_or_default();
        let table_name = table.name();

        let sql =
            if table.has_row_id() == Some(true) {
                &format!("
                    SELECT *
                    FROM {table_name}
                    WHERE rowid >= {offset}
                    ORDER BY rowid {row_order}
                    LIMIT {limit};
                ")
            } else {
                &format!("
                    SELECT *
                    FROM {table_name}
                    LIMIT {limit}
                    OFFSET {offset};
                ")
            };

        let connection = self.connection.borrow();

        let mut sql = connection.prepare(sql)?;
        let n_columns = sql.column_count();

        let iter = sql.query_map([], |row| {
            let mut values = Vec::new();

            for i in 0..n_columns {
                let value = match row.get_ref(i)? {
                    ValueRef::Null       => Affinity::NULL,
                    ValueRef::Integer(i) => Affinity::INTEGER(Some(i)),
                    ValueRef::Real(f)    => Affinity::REAL(Some(f)),
                    ValueRef::Text(t) =>
                        Affinity::TEXT(
                            Some(String::from_utf8_lossy(t).into())
                        ),
                    ValueRef::Blob(b) =>
                        Affinity::BLOB(
                            Some(b.len() as i32),
                            Some(hex_preview(b, BLOB_PREVIEW_LEN)
                        ),
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

impl Row {
    pub fn format_with(&self, separator: ColumnSeparator) -> String {
        let collection = self.cells
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>();

        match separator {
            ColumnSeparator::Tabs     => collection.join("\t"),
            ColumnSeparator::Spaces   => collection.join(" "),
            ColumnSeparator::Commas   => collection.join(","),
            ColumnSeparator::Markdown => format!("| {} |", collection.join(" | ")),
        }
    }
}


#[derive(Clone, Copy, Debug, Default)]
pub enum RowOrder {
    #[default]
    Descending,
    Ascending,
}

impl fmt::Display for RowOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Descending => "DESC",
            Self::Ascending  => "ASC",
        };

        write!(f, "{}", s)
    }
}


fn hex_preview(blob: &[u8], length: usize) -> String {
    blob
        .iter()
        .take(length)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
