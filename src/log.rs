//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::fmt::Debug;
use std::process;

use chrono::Utc;


pub fn log(message: &str) {
    println!("{}", message);
}

pub fn info(message: &str) {
    println!("{}", format_line(message));
}


pub fn debug(message: &str) {
    debug_base(&format_line(message));
}

pub fn debug_struct(s: &impl Debug) {
    debug_base(&format!("{:#?}", s));
}

pub fn debug_base(message: &str) {
    if let Ok("1") = env::var("DEBUG").as_deref() {
        println!("\x1b[2m{}\x1b[0m", message);
    }
}


pub fn error(message: &str) {
    let app = env!("CARGO_PKG_NAME");
    eprintln!("\x1b[31m{app} error:\x1b[0m {message}");
}

pub fn error_and_exit(message: &str) -> ! {
    let app = env!("CARGO_PKG_NAME");
    eprintln!("\x1b[31m{app} error:\x1b[0m {message}");
    process::exit(255);
}


fn format_line(message: &str) -> String {
    format!("{timestamp} | {message}", timestamp=format_timestamp())
}

fn format_timestamp() -> String {
    // Docs: https://docs.rs/chrono/latest/chrono/format/strftime

    let now = Utc::now();
    now.format("%H:%M:%S").to_string()
}
