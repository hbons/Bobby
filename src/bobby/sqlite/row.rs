//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;

use rusqlite::types::ValueRef;


use super::affinity::Affinity;
use super::column::{ self, ColumnSeparator };
use super::database::Database;
use super::table::Table;


#[derive(Clone, Debug, Default)]
pub struct Row {
    /// ROWID or the row's PRIMARY_KEY value if the
    /// table was created using WITHOUT_ROWID
    pub id: String, // TODO: Fill in. Option?
    pub cells: Vec<Affinity>,
}


impl Database {
    pub fn rows(
        &self,
        table: &Table,
        offset: Option<u32>,
        limit:  Option<u32>,
    ) -> Result<Vec<Row>, Box<dyn Error>>
    {
        const BLOB_PREVIEW_LEN: usize = 8;

        let limit = limit.unwrap_or(u32::MAX); // GTK models are limited to u32
        let offset = offset.unwrap_or(0);
        let row_order = self.row_order.unwrap_or_default();
        let table_name = table.name.clone();

        let filter = match table.filter {
            Some(ref f) =>
                column::as_filter_statement(
                    &self.columns(table).unwrap_or_default(),
                    f.into(),
                ),
            None => "TRUE".to_string(),
        };

        let sql =
            if table.has_row_id == Some(true) {
                &format!("
                    SELECT _rowid_, *
                    FROM {table_name}
                    WHERE rowid >= {offset}
                      AND {filter}
                    ORDER BY rowid {row_order}
                    LIMIT {limit};
                ")
            } else {
                &format!("
                    SELECT *
                    FROM {table_name}
                    WHERE {filter}
                    LIMIT {limit}
                    OFFSET {offset};
                ")
            };

        let mut sql = self.connection
            .borrow()
            .prepare(sql)?;

        let n_columns = sql.column_count();

        let iter = sql.query_map([], |row| {
            let mut r = Row::default();

            let mut first_column = 0;

            r.id = if let Some(true) = table.has_row_id {
                    first_column = 1;
                    row.get::<_, i64>(0)?.to_string()
                } else {
                    // need the primary key index here. use table.primary_key.id
                    row.get(table.primary_key
                        .clone()
                        .unwrap()
                        .name)
                };

            for i in first_column..n_columns {
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

                r.cells.push(value);
            }

            Ok(r)
        })?;

        Ok(iter.collect::<Result<Vec<_>, _>>()?)
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


fn hex_preview(
    blob: &[u8],
    len: usize,
) -> String
{
    blob
        .iter()
        .take(len)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}


#[derive(Clone, Copy, Debug, Default)]
pub enum RowOrder {
    #[default] Descending,
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
