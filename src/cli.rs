//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::process::exit;

use crate::app::{ app_deps, app_version, App };


impl App {
    pub fn cli_parse_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        match args.get(1).map(|s| s.as_str()) {
            Some("--help")    => self.cli_option_help(),
            Some("--version") => println!("{}", app_version()),
            Some("--deps")    => println!("{}", app_deps()),
            Some("--env")     => println!("{:#?}", self),
            None | Some(_)    => { return Ok(()); },
        }

        exit(0);
    }


    pub fn cli_option_help(&self) {
        println!("Usage: bobby [file]...");
        println!();
        println!("Options:");
        println!("    --help, --version, --deps, --env");
        println!();
    }
}
