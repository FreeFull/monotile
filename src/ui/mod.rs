use vizia::prelude::*;

mod actions;
mod canvas;
use self::canvas::{Canvas, Color, Tile};
mod views;
use self::views::*;
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
                    MenuButton::new_simple(cx, "New", |cx| cx.emit("app.new"));
                    MenuButton::new_simple(cx, "Open", |cx| cx.emit("app.open"));
                    MenuButton::new_simple(cx, "Save", |cx| cx.emit("app.save"));
                    MenuButton::new_simple(cx, "Save as", |cx| cx.emit("app.saveas"));
                    MenuButton::new_simple(cx, "Quit", |cx| cx.emit("app.quit"));
                },
            );
            Menu::new(
                cx,
                |cx| Label::new(cx, "Edit"),
                |cx| {
                    MenuButton::new_simple(cx, "Undo", |cx| {});
                    MenuButton::new_simple(cx, "Redo", |cx| {});
                    MenuButton::new_simple(cx, "Cut", |cx| {});
                    MenuButton::new_simple(cx, "Copy", |cx| {});
                    MenuButton::new_simple(cx, "Paste", |cx| {});
                },
            );

            Menu::new(
                cx,
                |cx| Label::new(cx, "Help"),
                |cx| {
                    MenuButton::new_simple(cx, "Monotile Help", |cx| {});
                    MenuButton::new_simple(cx, "About Monotile", |cx| {});
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
                println!("New file");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyO),
            KeymapEntry::new("app.open", |cx| {
                println!("Open file");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyS),
            KeymapEntry::new("app.save", |cx| {
                println!("Save file");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyS),
            KeymapEntry::new("app.saveas", |cx| {
                println!("Save file as");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyQ),
            KeymapEntry::new("app.quit", |cx| {
                println!("Quit");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyZ),
            KeymapEntry::new("app.undo", |cx| {
                println!("Undo");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyZ),
            KeymapEntry::new("app.redo", |cx| {
                println!("Redo");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyY),
            KeymapEntry::new("app.redo", |cx| {
                println!("Redo");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyX),
            KeymapEntry::new("app.cut", |cx| {
                println!("Cut");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyC),
            KeymapEntry::new("app.copy", |cx| {
                println!("Copy");
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyV),
            KeymapEntry::new("app.paste", |cx| {
                println!("Paste");
            }),
        ),
        (
            KeyChord::new(Modifiers::empty(), Code::F1),
            KeymapEntry::new("app.help", |cx| {
                println!("Help");
            }),
        ),
    ])
    .build(cx);
}

pub fn build(cx: &mut Context) {
    State::default().build(cx);
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
