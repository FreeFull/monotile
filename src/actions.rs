use std::io;

use crate::{App, Message};

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    New,
    Open,
    Save,
    SaveAs,
    Quit,
    Undo,
    Redo,
    Copy,
    Cut,
    Paste,
    Help,
    About,
    TileUp,
    TileLeft,
    TileDown,
    TileRight,
    TileIndex(u32),
}

impl cosmic::widget::menu::action::MenuAction for Action {
    type Message = crate::Message;

    fn message(&self) -> Self::Message {
        match self {
            Action::New => Message::Todo,
            Action::Open => Message::Todo,
            Action::Save => Message::Todo,
            Action::SaveAs => Message::Todo,
            Action::Quit => Message::CloseRequested,
            Action::Undo => Message::Todo,
            Action::Redo => Message::Todo,
            Action::Copy => Message::Todo,
            Action::Cut => Message::Todo,
            Action::Paste => Message::Todo,
            Action::Help => Message::Todo,
            Action::About => Message::Todo,
            Action::TileUp => Message::TileUp,
            Action::TileLeft => Message::TileLeft,
            Action::TileDown => Message::TileDown,
            Action::TileRight => Message::TileRight,
            Action::TileIndex(_) => Message::Todo,
        }
    }
}

pub fn save(state: &mut App) -> io::Result<()> {
    /*
    if state.file.handle.is_some() {
        file_formats::save(&state.file)
    } else {
        save_dialog(state)
    }*/
    unimplemented!()
}

fn save_dialog(state: &mut App) -> io::Result<()> {
    /*
    let mut dialog = rfd::FileDialog::new().add_filter("Monotile file", &[".monti"]);
    if let Some(path) = state.file.handle.as_ref().map(|h| &h.path) {
        dialog = dialog.set_directory(path);
    }
    if let Some(name) = state.file.handle.as_ref().map(|h| h.name()) {
        dialog = dialog.set_file_name(&*name);
    }
    if let Some(path) = dialog.save_file() {
        if state.file.handle.is_none() {
            state.file.handle = Some(Handle {
                path,
                file_type: file_formats::FileType::Monti,
            });
        }
        if let Err(err) = file_formats::save(&state.file) {
            rfd::MessageDialog::new()
                .set_level(rfd::MessageLevel::Error)
                .set_title("Error saving file")
                .set_description(&format!("Failed to save file: {:?}", err))
                .show();
            Err(err)
        } else {
            state.file.modified = false;
            Ok(())
        }
    } else {
        Ok(())
    }
    */
    unimplemented!()
}
