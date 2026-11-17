//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use crate::bobby::sqlite::affinity::Affinity;
use crate::bobby::sqlite::column::{ self, Column };
use crate::bobby::sqlite::column_name::ColumnName;


#[test]
fn test_column_name_parse() {
    let name = "column 1".parse::<ColumnName>().unwrap();

    assert_eq!(name.quoted(),   "\"column 1\"");
    assert_eq!(name.unquoted(), "column 1");


    let name = "column\" 2".parse::<ColumnName>().unwrap();

    assert_eq!(name.quoted(),   "\"column\"\" 2\"");
    assert_eq!(name.unquoted(), "column\" 2");


    let result = "".parse::<ColumnName>();
    assert!(result.is_err());

    let result = " ".parse::<ColumnName>();
    assert!(result.is_ok());
}


#[test]
fn test_column_as_filter_statement() {
    let column1 = Column {
        name: "column_1".parse::<ColumnName>().unwrap(),
        affinity: Affinity::TEXT(None),
        ..Default::default()
    };

    let column2 = Column {
        name: "column_2".parse::<ColumnName>().unwrap(),
        affinity: Affinity::TEXT(None),
        ..Default::default()
    };

    let column3 = Column {
        name: "column_3".parse::<ColumnName>().unwrap(),
        affinity: Affinity::NUMERIC(None),
        ..Default::default()
    };

    let column4 = Column {
        name: "column_4".parse::<ColumnName>().unwrap(),
        affinity: Affinity::BLOB(None, None),
        ..Default::default()
    };

    let filter = "hello".to_string();


    assert_eq!(
        column::as_filter_statement(&[column1.clone()], filter.clone()),
        "\"column_1\" LIKE '%hello%'"
    );

    assert_eq!(
        column::as_filter_statement(&[column1, column2, column3], filter.clone()),
        "\"column_1\" LIKE '%hello%' OR \"column_2\" LIKE '%hello%'"
    );

    assert_eq!(
        column::as_filter_statement(&[column4], filter),
        ""
    );
}
