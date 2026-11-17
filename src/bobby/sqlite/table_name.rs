//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::str;


#[derive(Clone, Debug, PartialEq)]
pub struct TableName(String);

impl str::FromStr for TableName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Table name is empty".into());
        }

        if s.starts_with("sqlite_") {
            return Err("Table name starts with sqlite_".into());
        }

        Ok(Self(s.to_string()))
    }
}


impl TableName {
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


impl Default for TableName {
    fn default() -> Self {
        Self("table".into())
    }
}
