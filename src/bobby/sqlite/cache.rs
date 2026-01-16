//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::rc::Rc;

use gio::glib;
use gio::glib::Object;
use gtk4::subclass::prelude::ObjectSubclassIsExt;

use super::database::Database;
use super::table::Table;


glib::wrapper! {
    pub struct DatabaseCacheModel(ObjectSubclass<imp::DatabaseCacheModelImpl>)
        @implements gio::ListModel;
}


impl DatabaseCacheModel {
    pub fn from_database(database: &Database, table: &Table) -> Self {
        let obj: Self = Object::new();
        let imp = obj.imp();

        *imp.database.borrow_mut() = Rc::new(database.clone());
        *imp.table.borrow_mut() = Rc::new(table.clone());

        obj
    }
}


mod imp {
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    use gio::prelude::*;
    use gio::subclass::prelude::*;
    use gio::subclass::prelude::ListModelImpl;

    use gio::glib;
    use gio::glib::BoxedAnyObject;

    use super::super::database::Database;
    use super::super::table::Table;


    const CACHE_PAGE_SIZE: u32 = 256;
    const GUARD_RADIUS: u32 = 256;


    #[derive(Default)]
    pub struct DatabaseCacheModelImpl {
        pub database: RefCell<Rc<Database>>,
        pub table: RefCell<Rc<Table>>,

        row_count: Cell<Option<u32>>,
        cached_rows: RefCell<BTreeMap<u32, BoxedAnyObject>>,
    }


    #[glib::object_subclass]
    impl ObjectSubclass for DatabaseCacheModelImpl {
        const NAME: &'static str = "DatabaseCacheModel";

        type Type = super::DatabaseCacheModel;
        type ParentType = glib::Object;
        type Interfaces = (gio::ListModel,);
    }


    impl ObjectImpl for DatabaseCacheModelImpl {}

    impl ListModelImpl for DatabaseCacheModelImpl {
        fn item(&self, index: u32) -> Option<glib::Object> {
            // println!("item({index})");

            if !self.cached_rows.borrow().contains_key(&index) {
                // println!("Caching {CACHE_PAGE_SIZE} rows around {index}");

                let database = self.database.borrow();
                let table = self.table.borrow();

                let offset: u32 = index.saturating_sub(CACHE_PAGE_SIZE / 2);

                let rows = database.rows(
                    &table,
                    Some(offset),
                    Some(CACHE_PAGE_SIZE),
                ).ok()?;

                {
                    let mut cached_rows = self.cached_rows.borrow_mut();

                    for (i, row) in rows.iter().enumerate() {
                        let boxed_row = BoxedAnyObject::new(row.clone());
                        cached_rows.insert(offset + i as u32, boxed_row);
                    }

                    let lo_guard = index.saturating_sub(GUARD_RADIUS);
                    let hi_guard = index + GUARD_RADIUS;
                    cached_rows.retain(|i, _| *i >= lo_guard && *i <= hi_guard);
                }
            }

            let boxed_row = {
                let cache = self.cached_rows.borrow();
                cache.get(&index)?.clone()
            };

            Some(boxed_row.into())
        }


        fn n_items(&self) -> u32 {
            // println!("n_items()");

            if let Some(count) = self.row_count.get() {
                return count;
            }

            let database = self.database.borrow();
            let table = self.table.borrow();

            let count = database.row_count(&table).unwrap_or(0);
            self.row_count.set(Some(count));

            count
        }


        fn item_type(&self) -> glib::Type {
            // println!("item_type()");
            BoxedAnyObject::static_type()
        }
    }
}
