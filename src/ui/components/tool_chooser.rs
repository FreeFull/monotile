use ui::state::{State, Tool};

use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, AccelFlags, AccelGroup, Image, Orientation, RadioButton};

use gdk::enums::key;
use gdk::ModifierType;

pub fn build(state: &Rc<State>) -> gtk::Box {
    let icon_size = gtk::IconSize::LargeToolbar.into();
    let toolbar = gtk::Box::new(Orientation::Horizontal, 0);
    let draw = RadioButton::new();
    let draw_icon = Image::new_from_icon_name("document-edit-symbolic", icon_size);
    draw.set_label("Draw");
    draw.set_image(&draw_icon);
    draw.set_always_show_image(true);
    draw.set_tooltip_markup("Draw <b>R</b>");
    draw.set_property_draw_indicator(false);
    draw.connect_clicked({
        let state = state.clone();
        move |_| {
            state.current_tool.replace(Tool::Draw);
        }
    });
    let flood = RadioButton::new_from_widget(&draw);
    let flood_icon = Image::new_from_icon_name("edit-clear-all-symbolic", icon_size);
    flood.set_property_draw_indicator(false);
    flood.set_label("Flood fill");
    flood.set_image(&flood_icon);
    flood.set_always_show_image(true);
    flood.set_tooltip_markup("Flood fill <b>F</b>");
    flood.connect_clicked({
        let state = state.clone();
        move |_| {
            state.current_tool.replace(Tool::FloodFill);
        }
    });

    let group = AccelGroup::new();
    state.window.add_accel_group(&group);

    draw.add_accelerator(
        "activate",
        &group,
        key::R,
        ModifierType::empty(),
        AccelFlags::VISIBLE,
    );
    flood.add_accelerator(
        "activate",
        &group,
        key::F,
        ModifierType::empty(),
        AccelFlags::VISIBLE,
    );

    toolbar.add(&draw);
    toolbar.add(&flood);
    toolbar
}
