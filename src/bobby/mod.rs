//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


pub mod sqlite {
    pub mod affinity;
    pub mod cache;
    pub mod column;
    pub mod column_name;
    pub mod database;
    pub mod row;
    pub mod row_order;
    pub mod table;
    pub mod table_name;
}

pub mod prelude;

#[cfg(test)]
pub mod tests;
