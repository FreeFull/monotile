use super::actions::Action;
use vizia::prelude::*;

pub fn build(cx: &mut Context) {
    MenuController::new(cx, true, |cx| {
        MenuStack::new_horizontal(cx, |cx| {
            Menu::new(
                cx,
                |cx| Label::new(cx, "File"),
                |cx| {
                    MenuButton::new_simple(cx, "New", |cx| cx.emit(Action::New));
                    MenuButton::new_simple(cx, "Open", |cx| cx.emit(Action::Open));
                    MenuButton::new_simple(cx, "Save", |cx| cx.emit(Action::Save));
                    MenuButton::new_simple(cx, "Save as", |cx| cx.emit(Action::SaveAs));
                    MenuButton::new_simple(cx, "Quit", |cx| cx.emit(Action::Quit));
                },
            );
            Menu::new(
                cx,
                |cx| Label::new(cx, "Edit"),
                |cx| {
                    MenuButton::new_simple(cx, "Undo", |cx| cx.emit(Action::Undo));
                    MenuButton::new_simple(cx, "Redo", |cx| cx.emit(Action::Redo));
                    MenuButton::new_simple(cx, "Copy", |cx| cx.emit(Action::Copy));
                    MenuButton::new_simple(cx, "Cut", |cx| cx.emit(Action::Cut));
                    MenuButton::new_simple(cx, "Paste", |cx| cx.emit(Action::Paste));
                },
            );
            Menu::new(
                cx,
                |cx| Label::new(cx, "Help"),
                |cx| {
                    MenuButton::new_simple(cx, "Monotile Help", |cx| cx.emit(Action::Help));
                    MenuButton::new_simple(cx, "About Monotile", |cx| cx.emit(Action::About));
                },
            );
            MenuButton::new_simple(cx, "Test Quit Dialog", |cx| {
                cx.emit(super::dialogs::StateSetter::Quit(true))
            });
        });
    });
}
