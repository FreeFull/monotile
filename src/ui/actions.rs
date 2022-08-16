use std::io;

use crate::ui::file_formats;
use crate::ui::state::*;

#[derive(Copy, Clone, Hash, Debug)]
pub enum Action {
    New,
    Open,
    Save,
    SaveAs,
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
}

pub fn save(state: &mut State) -> io::Result<()> {
    if let Some(ref path) = state.file_path {
        file_formats::save(path, &state.canvas)
    } else {
        save_dialog(state)
    }
}

fn save_dialog(state: &mut State) -> io::Result<()> {
    let mut dialog = rfd::FileDialog::new().add_filter("Monotile file", &[".monti"]);
    if let Some(ref path) = state.file_path {
        dialog = dialog.set_directory(path);
    }
    if let Some(ref name) = state.file_name {
        dialog = dialog.set_file_name(name);
    }
    if let Some(path) = dialog.save_file() {
        if let Err(err) = file_formats::save(path, &state.canvas) {
            rfd::MessageDialog::new()
                .set_level(rfd::MessageLevel::Error)
                .set_title("Error saving file")
                .set_description(&format!("Failed to save file: {:?}", err))
                .show();
            Err(err)
        } else {
            state.modified = false;
            Ok(())
        }
    } else {
        Ok(())
    }
}
