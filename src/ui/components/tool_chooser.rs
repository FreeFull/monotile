use ui::state::{State, Tool};

use std::rc::Rc;

use gtk::prelude::*;
use gtk::{RadioToolButton, Toolbar};

pub fn build(state: &Rc<State>) -> Toolbar {
    let toolbar = Toolbar::new();
    let draw = RadioToolButton::new();
    draw.set_icon_name("document-edit-symbolic");
    draw.set_label("Draw");
    draw.set_tooltip_text("Draw");
    draw.connect_clicked({
        let state = state.clone();
        move |_| {
            state.current_tool.replace(Tool::Draw);
        }
    });
    let flood = RadioToolButton::new_from_widget(&draw);
    flood.set_icon_name("edit-clear-all-symbolic");
    flood.set_label("Flood fill");
    flood.set_tooltip_text("Flood fill");
    flood.connect_clicked({
        let state = state.clone();
        move |_| {
            state.current_tool.replace(Tool::FloodFill);
        }
    });

    toolbar.insert(&draw, -1);
    toolbar.insert(&flood, -1);
    toolbar
}
