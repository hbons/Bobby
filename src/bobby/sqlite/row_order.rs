//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::fmt;


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
