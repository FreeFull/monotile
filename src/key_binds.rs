use cosmic::{
    iced_core::keyboard::Key,
    widget::menu::key_bind::{KeyBind, Modifier},
};
use std::collections::HashMap;

use crate::Action;

pub fn key_binds() -> HashMap<KeyBind, Action> {
    HashMap::from([
        (
            KeyBind {
                modifiers: vec![Modifier::Ctrl],
                key: Key::Character("n".into()),
            },
            Action::New,
        ),
        (
            KeyBind {
                modifiers: vec![Modifier::Ctrl],
                key: Key::Character("q".into()),
            },
            Action::Quit,
        ),
        (
            KeyBind {
                modifiers: vec![],
                key: Key::Character("w".into()),
            },
            Action::TileUp,
        ),
        (
            KeyBind {
                modifiers: vec![],
                key: Key::Character("a".into()),
            },
            Action::TileLeft,
        ),
        (
            KeyBind {
                modifiers: vec![],
                key: Key::Character("s".into()),
            },
            Action::TileDown,
        ),
        (
            KeyBind {
                modifiers: vec![],
                key: Key::Character("d".into()),
            },
            Action::TileRight,
        ),
    ])
}
