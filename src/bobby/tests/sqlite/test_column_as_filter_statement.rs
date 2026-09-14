//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use crate::bobby::sqlite::column::{ Column, as_filter_statement };
use crate::bobby::sqlite::affinity::Affinity;


#[test]
fn test_column_as_filter_statement() {
    let column1 = Column {
        name: "column_1".to_string(),
        affinity: Affinity::TEXT(None),
        ..Default::default()
    };

    let column2 = Column {
        name: "column_2".to_string(),
        affinity: Affinity::TEXT(None),
        ..Default::default()
    };

    let column3 = Column {
        name: "column_3".to_string(),
        affinity: Affinity::NUMERIC(None),
        ..Default::default()
    };

    let column4 = Column {
        name: "column_4".to_string(),
        affinity: Affinity::BLOB(None, None),
        ..Default::default()
    };

    let filter = "hello".to_string();


    assert_eq!(
        as_filter_statement(&vec![column1.clone()], filter.clone()),
        "column_1 LIKE '%hello%'"
    );

    assert_eq!(
        as_filter_statement(&vec![column1, column2, column3], filter.clone()),
        "column_1 LIKE '%hello%' OR column_2 LIKE '%hello%'"
    );

    assert_eq!(
        as_filter_statement(&vec![column4], filter),
        ""
    );
}
