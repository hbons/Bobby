//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::str;


#[derive(Clone, Debug, PartialEq)]
pub struct ColumnName(String);

impl str::FromStr for ColumnName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Column name is empty".into());
        }

        Ok(Self(s.to_string()))
    }
}


impl ColumnName {
    /// For use in queries
    pub fn quoted(&self) -> String {
        let s = self.0.replace("\"", "\"\"");
        format!("\"{s}\"")
    }

    /// For use in user interface controls
    pub fn unquoted(&self) -> String {
        self.0.clone()
    }
}


impl Default for ColumnName {
    fn default() -> Self {
        Self("column".into())
    }
}
