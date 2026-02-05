//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


pub mod actions {
    pub mod app_about;
    pub mod app_close;
    pub mod app_open;
    pub mod app_preferences;
    pub mod app_quit;
    pub mod app_shortcuts;
    pub mod prelude;
}

pub mod dialogs {
    pub mod about;
    // pub mod jump;
    pub mod preferences;
    pub mod shortcuts;
}

pub mod windows {
    pub mod window;
    pub mod window_empty;
    pub mod window_error;
}

pub mod content;
pub mod drop_target;
pub mod files;
pub mod item;
pub mod lib;
pub mod menu;
pub mod switcher;
