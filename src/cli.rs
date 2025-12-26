//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use crate::app::{ app_version, App };


impl App {
    pub async fn cli_parse_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(1, args)?;
        let file = args.get(1).ok_or("Missing <file>")?;

        match file.as_str() {
            "--help" => self.cli_option_help(),
            "--version" => println!("{}", app_version()),
            "--env" => println!("{:#?}", self),
            path => {
                self.cli_command_open(path)?;
            }
        }

        Ok(())
    }

    pub fn cli_option_help(&self) {
        println!("Usage: bobby <file>");
        println!();
        println!("Options:");
        println!("    --help, --version, --env");
        println!();
    }
}


impl App {
    pub fn cli_command_open(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = std::fs::read_to_string(path)?;
        println!("{}", contents);

        Ok(())
    }
}


impl App {
    /// Checks if the minimum amount of args have been passed
    pub(crate) fn cli_require_args(&self, count: usize, args: &[String]) -> Result<(), Box<dyn Error>> {
        if args.len() - 1 < count {
            self.cli_option_help();
            return Err(format!("Command requires {count} arguments").into());
        }

        Ok(())
    }
}
