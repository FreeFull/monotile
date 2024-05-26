use std::collections::HashMap;

use cosmic::widget::menu::{self, items, root, Item, MenuBar, Tree};
use cosmic::Element;

use crate::{Action, Message};

pub fn menu_bar(key_binds: &HashMap<menu::key_bind::KeyBind, Action>) -> Element<'static, Message> {
    MenuBar::new(vec![
        Tree::with_children(
            root("File"),
            items(
                key_binds,
                vec![
                    Item::Button("New", Action::New),
                    Item::Button("Open", Action::Open),
                    Item::Button("Save", Action::Save),
                    Item::Button("Save As", Action::SaveAs),
                    Item::Button("Quit", Action::Quit),
                ],
            ),
        ),
        Tree::with_children(
            root("Edit"),
            items(
                key_binds,
                vec![
                    Item::Button("Undo", Action::Undo),
                    Item::Button("Redo", Action::Redo),
                    Item::Divider,
                    Item::Button("Cut", Action::Cut),
                    Item::Button("Copy", Action::Copy),
                    Item::Button("Paste", Action::Paste),
                ],
            ),
        ),
    ])
    .into()
}
