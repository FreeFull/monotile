use std::{io, path::Path};

use crate::{
    canvas::Canvas,
    file_formats::{load, Handle},
};

#[derive(Debug, Default)]
pub struct FileState {
    pub handle: Option<Handle>,
    pub canvas: Canvas,
    pub modified: bool,
}

impl FileState {
    pub fn new(path: Option<impl AsRef<Path>>) -> io::Result<FileState> {
        if let Some(path) = path {
            load(path)
        } else {
            Ok(FileState {
                handle: None,
                canvas: Canvas::default(),
                modified: false,
            })
        }
    }
}
