//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::str;


#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Affinity {
    #[default]
    NUMERIC,
    INTEGER,
    REAL,
    TEXT,
    BLOB,
}


impl str::FromStr for Affinity {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .split('(') // Could be "VARCHAR(255)"
            .next()
            .ok_or("Empty string")?
            .to_uppercase();

        // Docs: https://www.sqlite.org/datatype3.html
        match () {
            _ if s.contains("INT") => Ok(Self::INTEGER),
            _ if s.contains("TEXT") || s.contains("CHAR") || s.contains("CLOB") => Ok(Self::TEXT),
            _ if s.contains("BLOB") || s.is_empty() => Ok(Self::BLOB),
            _ if s.contains("REAL") || s.contains("FLOA") || s.contains("DOUB") => Ok(Self::REAL),
            _ => Ok(Self::NUMERIC),
        }
    }
}
