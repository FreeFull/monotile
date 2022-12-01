use super::actions::Action;
use vizia::{modifiers::EventHandle, prelude::*};

#[derive(Copy, Clone, Default, Lens, Model, Setter)]
pub struct State {
    pub quit: bool,
    pub color_picker: Option<usize>,
}

pub fn build(cx: &mut Context) {
    State {
        quit: false,
        color_picker: None,
    }
    .build(cx);
    dialog(
        cx,
        State::quit,
        |cx| cx.emit(StateSetter::Quit(false)),
        |cx| {
            Label::new(
                cx,
                "There are unsaved changes.\nAre you sure you want to quit?",
            );
            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |_cx| {
                        std::process::exit(0);
                    },
                    |cx| Label::new(cx, "Quit without saving"),
                )
                .class("danger");
                Button::new(
                    cx,
                    |cx| {
                        cx.emit(Action::Save);
                        cx.emit(Action::Quit);
                    },
                    |cx| Label::new(cx, "Save and quit"),
                )
                .class("confirm");
                Button::new(
                    cx,
                    |cx| cx.emit(StateSetter::Quit(false)),
                    |cx| Label::new(cx, "Cancel"),
                );
            });
        },
    );
    dialog(
        cx,
        State::color_picker.map(|index| index.is_some()),
        |cx| cx.emit(StateSetter::ColorPicker(None)),
        |cx| {
            Label::new(cx, "Colour Picker");
        },
    );
}

fn dialog(
    cx: &mut Context,
    lens: impl Lens<Target = bool>,
    dismiss: for<'a, 'b> fn(&mut EventHandle<'a, 'b, Element>),
    contents: impl Fn(&mut Context) + Copy + 'static,
) -> Handle<impl View> {
    Popup::new(cx, lens, true, move |cx| {
        ZStack::new(cx, |cx| {
            Element::new(cx)
                .class("dialog-background")
                .on_press(dismiss);
            VStack::new(cx, contents).class("dialog");
        });
    })
    .class("dialog-popup")
}
