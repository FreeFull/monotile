use crate::ui::state::{State, Tool};

use vizia::prelude::*;

pub fn build(cx: &mut Context) {
    let _toolbar = HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            RadioButton::new(cx, State::current_tool.map(|&tool| tool == Tool::Draw))
                .on_press(|cx| cx.emit(Tool::Draw));
            Label::new(cx, "Draw");
        });
        HStack::new(cx, |cx| {
            RadioButton::new(cx, State::current_tool.map(|&tool| tool == Tool::FloodFill))
                .on_press(|cx| cx.emit(Tool::FloodFill));
            Label::new(cx, "Flood Fill");
        });
    });
}
