use std::path::Path;

use vizia::prelude::*;

mod actions;
mod canvas;
mod views;
use self::{actions::Action, views::*};
mod file_formats;
mod state;
mod tileset;
pub use self::state::State;

fn build_menu(cx: &mut Context) {
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
                    MenuButton::new_simple(cx, "Quit", |cx| {
                        todo!("Vizia doesn't have a way of breaking its event loop yet.");
                    });
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
        });
    });
}

fn add_accelerators(cx: &mut Context) {
    Keymap::from(vec![
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyN),
            KeymapEntry::new("app.new", |cx| {
                cx.emit(Action::New);
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyO),
            KeymapEntry::new("app.open", |cx| cx.emit(Action::Open)),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyS),
            KeymapEntry::new("app.save", |cx| cx.emit(Action::Save)),
        ),
        (
            KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyS),
            KeymapEntry::new("app.saveas", |cx| cx.emit(Action::SaveAs)),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyQ),
            KeymapEntry::new("app.quit", |cx| cx.emit(WindowEvent::WindowClose)),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyZ),
            KeymapEntry::new("app.undo", |cx| cx.emit(Action::Undo)),
        ),
        (
            KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyZ),
            KeymapEntry::new("app.redo", |cx| cx.emit(Action::Redo)),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyY),
            KeymapEntry::new("app.redo", |cx| cx.emit(Action::Redo)),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyC),
            KeymapEntry::new("app.copy", |cx| cx.emit(Action::Copy)),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyX),
            KeymapEntry::new("app.cut", |cx| cx.emit(Action::Cut)),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyV),
            KeymapEntry::new("app.paste", |cx| cx.emit(Action::Paste)),
        ),
        (
            KeyChord::new(Modifiers::empty(), Code::F1),
            KeymapEntry::new("app.help", |cx| cx.emit(Action::Help)),
        ),
        (
            KeyChord::new(Modifiers::empty(), Code::KeyW),
            KeymapEntry::new("app.w", |cx| cx.emit(Action::TileUp)),
        ),
        (
            KeyChord::new(Modifiers::empty(), Code::KeyA),
            KeymapEntry::new("app.w", |cx| cx.emit(Action::TileLeft)),
        ),
        (
            KeyChord::new(Modifiers::empty(), Code::KeyS),
            KeymapEntry::new("app.w", |cx| cx.emit(Action::TileDown)),
        ),
        (
            KeyChord::new(Modifiers::empty(), Code::KeyD),
            KeymapEntry::new("app.w", |cx| cx.emit(Action::TileRight)),
        ),
    ])
    .build(cx);
}

pub fn build(cx: &mut Context, file: Option<impl AsRef<Path>>) {
    State::new(file, cx);
    add_accelerators(cx);
    build_menu(cx);
    HStack::new(cx, |cx| {
        drawing_area::build(cx);
        VStack::new(cx, |cx| {
            tile_chooser::build(cx);
            color_chooser::build(cx);
            tool_chooser::build(cx);
        })
        .width(Units::Auto);
    });
}
