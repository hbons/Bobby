//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::cell::RefCell;
use std::error::Error;

use gtk4::glib;
use gtk4::glib::subclass::prelude::*;
use rusqlite::types::ValueRef;

use super::database::Database;
use super::table::Table;


mod imp {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Default)]
    pub struct Row {
        pub cells: RefCell<Vec<String>>,
    }


    #[glib::object_subclass]
    impl ObjectSubclass for Row {
        const NAME: &'static str = "Row";
        type Type = super::Row;
    }


    impl ObjectImpl for Row {}
}


glib::wrapper! {
    pub struct Row(ObjectSubclass<imp::Row>);
}


impl Row {
    pub fn new(cells: Vec<String>) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().cells.replace(cells);
        obj
    }


    pub fn cells(&self) -> Vec<String> {
        self.imp().cells.borrow().clone()
    }
}


impl Database {
    pub fn rows(&self, table: &Table) -> Result<Vec<Row>, Box<dyn Error>> {
        let mut sql = self.connection.prepare(
            &format!("SELECT * FROM {};", table.name())
        )?;

        let n_columns = sql.column_count();

        let iter = sql.query_map([], |row| {
            let mut values = Vec::new();

            for i in 0..n_columns {
                let value = match row.get_ref(i)? {
                    ValueRef::Null       => "NULL".to_string(),
                    ValueRef::Integer(i) => i.to_string(),
                    ValueRef::Real(f)    => f.to_string(),
                    ValueRef::Text(t)    => String::from_utf8_lossy(t).into(),
                    ValueRef::Blob(b)    => format!("{}: {} …", b.len(), hex_preview(b, 8)),
                };

                values.push(value);
            }

            Ok(values)
        })?;

        Ok(iter
            .map(|cells| cells.map(Row::new))
            .collect::<Result<Vec<_>, _>>()?
        )
    }
}


pub fn hex_preview(blob: &[u8], length: usize) -> String {
    blob.iter()
        .take(length)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
