//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::cell::Ref;
use std::error::Error;

use gtk4::prelude::*;
use gtk4::{
    glib::BoxedAnyObject,
    glib::Object,
    // pango::EllipsizeMode,
    Align,
    Label,
    ListItem,
};

use crate::bobby::prelude::*;


// U+25C7 "White Diamond"
pub const MARGIN_END: i32 = 4;

pub fn setup_index_list_item(obj: &Object) -> Result<(), Box<dyn Error>> {
    let list_item = obj
        .downcast_ref::<ListItem>()
        .ok_or("Object is not a gtk4::ListItem")?;

    let label = Label::builder()
        .css_classes(["dimmed", "monospace"])
        .halign(Align::End)
        .has_tooltip(true)
        .margin_end(MARGIN_END)
        .margin_top(1)
        .single_line_mode(true)
        .build();

    list_item.set_child(Some(&label));

    Ok(())
}


pub fn bind_index_list_item(
    obj: &Object,
    row_count: u32,
) -> Result<(), Box<dyn Error>>
{
    let list_item = obj
        .downcast_ref::<ListItem>()
        .ok_or("Object is not a gtk4::ListItem")?;

    let label = list_item
        .child()
        .and_downcast::<Label>()
        .ok_or("Object is not a gtk4::Label")?;

    let row_number = list_item.position() as i64 + 1;
    let text = row_number.to_string();

    // Possible cell reuse
    if label.text() != text {
        label.set_text(&text);
    }

    if let Some(parent) = label.parent() {
        let tooltip_text = format!("{row_number} / {row_count}");

        // Possible cell reuse
        if parent.tooltip_text().as_deref() != Some(&tooltip_text) {
            parent.set_tooltip_text(Some(&tooltip_text));
        }
    }

    Ok(())
}


// U+25C7 "White Diamond"
pub const SYMBOL_KEY: &str = "◇";

pub fn setup_list_item(obj: &Object, monospace_font: bool) -> Result<(), Box<dyn Error>> {
    let list_item = obj
        .downcast_ref::<ListItem>()
        .ok_or("Object is not a gtk4::ListItem")?;

    let label = Label::builder()
        .css_classes(["numeric"])
        // .ellipsize(EllipsizeMode::End) // TODO: Still a bit slow...
        .halign(Align::Start)
        .has_tooltip(true)
        .margin_start(4)
        .single_line_mode(true)
        .build();

    // TODO: Bind setting
    if monospace_font {
        label.add_css_class("monospace");
        label.set_margin_top(1);
    }

    list_item.set_child(Some(&label));

    Ok(())
}


pub fn bind_list_item(
    obj: &Object,
    column_index: usize,
    primary_key: bool,
) -> Result<(), Box<dyn Error>>
{
    let list_item = obj
        .downcast_ref::<ListItem>()
        .ok_or("Object is not a gtk4::ListItem")?;

    let boxed = list_item
        .item()
        .and_downcast::<BoxedAnyObject>()
        .ok_or("Object is not a glib::BoxedAnyObject")?;

    let label = list_item
        .child()
        .and_downcast::<Label>()
        .ok_or("Object is not a gtk4::Label")?;

    let row: Ref<Row> = boxed.borrow();
    let cell = row.cells
        .get(column_index - 1)
        .ok_or("Missing column")?;


    let text = cell.to_string();

    // Possible cell reuse
    if label.text() != text {
        label.set_text(&text);
    }


    let name = column_index.to_string();

    // Possible cell reuse
    if label.widget_name() != name {
        label.set_widget_name(&name);
    }


    let dimmed = matches!(cell,
        Affinity::NULL | Affinity::NUMERIC(None) | Affinity::BLOB(_, _)
    );

    if dimmed {
        label.add_css_class("dimmed");
    } else {
        label.remove_css_class("dimmed");
    }


    if let Some(parent) = label.parent() {
        let tooltip_text = {
            let s = match cell {
                Affinity::NUMERIC(Some(s)) => format!("{} {s}", cell.to_type_string()),
                Affinity::INTEGER(Some(i)) => format!("{} {i}", cell.to_type_string()),
                Affinity::REAL(Some(f)) => format!("{} {f}", cell.to_type_string()),
                Affinity::TEXT(Some(s)) => format!("{} {s}", cell.to_type_string()),
                Affinity::BLOB(Some(_), Some(preview)) => format!(
                    "{} {preview} …", cell.to_type_string()
                ),
                Affinity::NULL => "NULL".to_string(),
                Affinity::NUMERIC(None) => "NULL".to_string(),
                _ => format!("{cell} NULL"),
            };

            if primary_key {
                format!("{SYMBOL_KEY} PRIMARY_KEY  {s}")
            } else {
                s
            }
        };

        // Possible cell reuse
        if parent.tooltip_text().as_deref() != Some(&tooltip_text) {
            parent.set_tooltip_text(Some(&tooltip_text));
        }
    }

    Ok(())
}
