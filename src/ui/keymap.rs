use super::actions::Action;
use vizia::prelude::*;

pub fn build(cx: &mut Context) {
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
