use crate::ui::file_formats;
use crate::ui::state::*;

#[derive(Copy, Clone, Hash, Debug)]
pub enum Action {
    New,
    Save,
    SaveAs,
    Load,
}

pub fn save(state: &mut State) {}

fn save_dialog(state: &mut State) {
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
        } else {
            state.modified = false;
        }
    }
}