//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use crate::bobby::sqlite::affinity::Affinity;


#[test]
fn test_sqlite_affinity_to_type_string() {
    assert_eq!(Affinity::NUMERIC(None).to_type_string(), "NUMERIC");
    assert_eq!(Affinity::INTEGER(None).to_type_string(), "INTEGER");
    assert_eq!(Affinity::REAL(None).to_type_string(), "REAL");
    assert_eq!(Affinity::TEXT(None).to_type_string(), "TEXT");
    assert_eq!(Affinity::BLOB(None, None).to_type_string(), "BLOB");
    assert_eq!(Affinity::REAL(None).to_type_string(), "NULL");
}
