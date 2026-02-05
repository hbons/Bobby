//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


pub mod actions {
    pub mod prelude;
    pub mod app_about;
    pub mod app_close;
    pub mod app_open;
    pub mod app_preferences;
    pub mod app_quit;
    pub mod app_shortcuts;
    pub mod win_copy_val;
    pub mod win_copy_row;
    pub mod win_switch_table;
}

pub mod dialogs {
    pub mod about;
    // pub mod jump;
    pub mod file;
    pub mod preferences;
    pub mod shortcuts;
}

pub mod widgets {
    pub mod button;
    pub mod content;
    pub mod drop_target;
    pub mod item;
    pub mod menu;
    pub mod switcher;
}

pub mod windows {
    pub mod prelude;
    pub mod window;
    pub mod window_empty;
    pub mod window_error;
}

pub mod lib;
