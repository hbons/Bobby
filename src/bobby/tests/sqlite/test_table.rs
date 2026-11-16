//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use crate::bobby::sqlite::table::TableName;


#[test]
fn test_table_name_parse() {
    let name = "table 1".parse::<TableName>().unwrap();

    assert_eq!(name.quoted(),   "\"table 1\"");
    assert_eq!(name.unquoted(), "table 1");


    let name = "table\" 2".parse::<TableName>().unwrap();

    assert_eq!(name.quoted(),   "\"table\"\" 2\"");
    assert_eq!(name.unquoted(), "table\" 2");


    let result = "".parse::<TableName>();
    assert!(result.is_err());

    let result = " ".parse::<TableName>();
    assert!(result.is_ok());

    let result = "sqlite_table".parse::<TableName>();
    assert!(result.is_err());
}
