//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::str;


#[derive(Clone, Debug, PartialEq)]
pub enum Affinity {
    NUMERIC(Option<String>),
    INTEGER(Option<i64>),
    REAL(Option<f64>),
    TEXT(Option<String>),
    BLOB(Option<i32>, Option<Vec<u8>>),
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
            _ if s.contains("INT") => Ok(Self::INTEGER(None)),
            _ if s.contains("TEXT") || s.contains("CHAR") || s.contains("CLOB") => Ok(Self::TEXT(None)),
            _ if s.contains("BLOB") || s.is_empty() => Ok(Self::BLOB(None, None)),
            _ if s.contains("REAL") || s.contains("FLOA") || s.contains("DOUB") => Ok(Self::REAL(None)),
            _ => Ok(Self::NUMERIC(None)),
        }
    }
}


impl fmt::Display for Affinity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NUMERIC(_) => "NUMERIC",
            Self::INTEGER(_) => "INTEGER",
            Self::REAL(_)    => "REAL",
            Self::TEXT(_)    => "TEXT",
            Self::BLOB(_, _) => "BLOB",
        };

        write!(f, "{}", s)
    }
}


impl Default for Affinity {
    fn default() -> Self {
        Affinity::NUMERIC(None)
    }
}
