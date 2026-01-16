//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::str;


#[derive(Clone, Debug, Default, PartialEq)]
pub enum Affinity {
    NUMERIC(Option<String>),
    INTEGER(Option<i64>),
    REAL(Option<f64>),
    TEXT(Option<String>),
    BLOB(Option<i32>, Option<String>),
    #[default] NULL,
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
            Self::NUMERIC(Some(s)) => s.to_string(),
            Self::INTEGER(Some(i)) => i.to_string(),
            Self::REAL(Some(f)) => f.to_string(),
            Self::TEXT(Some(s)) => s.to_string(),
            Self::BLOB(Some(length), _) => format!("{length} BYTES"),
            Self::NULL | Self::NUMERIC(None) => "—".to_string(),
            _ => "???".to_string(),
        };

        write!(f, "{}", s)
    }
}


impl Affinity {
    pub fn to_type_string(&self) -> &'static str {
        match self {
            Self::NUMERIC(_) => "NUMERIC",
            Self::INTEGER(_) => "INTEGER",
            Self::REAL(_) => "REAL",
            Self::TEXT(_) => "TEXT",
            Self::BLOB(_, _) => "BLOB",
            Self::NULL => "NULL",
        }
    }
}
