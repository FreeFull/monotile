use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use gdk::WindowExt;
use gio::prelude::*;
use gio::{self, MenuExt};
use glib::translate::ToGlib;
use gtk::prelude::*;
use gtk::{self, FileChooserAction, FileChooserNative, FileFilter, Orientation};

mod canvas;
use self::canvas::{Canvas, Color, Tile};
mod components;
use self::components::*;
mod file_formats;
mod tileset;

#[derive(Debug)]
pub struct State {
    pub open_file: RefCell<Option<PathBuf>>,
    pub canvas: RefCell<Canvas>,
    pub canvas_cursor_position: RefCell<Option<(usize, usize)>>,
    pub tileset: tileset::Tileset,
    pub current_tile: RefCell<Tile>,
}

fn build_menu(app: &gtk::Application) {
    let menu = gio::Menu::new();
    menu.append("New", "app.new");
    menu.append("Open", "app.open");
    menu.append("Save", "app.save");
    menu.append("Save as", "app.saveas");

    menu.append("Quit", "app.quit");

    app.set_app_menu(&menu);
}

fn add_actions(app: &gtk::Application, window: &gtk::ApplicationWindow, state: &Rc<State>) {
    let new = gio::SimpleAction::new("new", None);
    new.connect_activate({
        let state = state.clone();
        let app = app.clone();
        move |_, _| {
            state.open_file.replace(None);
            state.canvas.replace(Canvas::default());
            app.activate_action("file_changed", None);
        }
    });

    let open = gio::SimpleAction::new("open", None);
    open.connect_activate({
        let app = app.clone();
        let window = window.clone();
        move |_, _| {
            let dialog =
                FileChooserNative::new(None, Some(&window), FileChooserAction::Open, None, None);
            dialog.set_show_hidden(false);

            let filter = FileFilter::new();
            FileFilterExt::set_name(&filter, "Monotile file");
            filter.add_pattern("*.monti");
            dialog.add_filter(&filter);

            let app = app.clone();
            dialog.connect_response(move |dialog, resp| {
                if resp == gtk::ResponseType::Accept.to_glib() {
                    let files = dialog.get_files();
                    if files.len() > 0 {
                        app.open(&files, "");
                    }
                }
            });
            dialog.run();
        }
    });

    let save = gio::SimpleAction::new("save", None);
    save.connect_activate({
        let state = state.clone();
        let app = app.clone();
        move |_, _| match *state.open_file.borrow() {
            Some(ref path) => {
                let canvas = state.canvas.borrow();
                match file_formats::save(&path, &canvas) {
                    Ok(_) => {}
                    Err(err) => println!("save failed: {}", err),
                }
            }
            None => {
                app.activate_action("saveas", None);
            }
        }
    });
    let saveas = gio::SimpleAction::new("saveas", None);
    saveas.connect_activate({
        let app = app.clone();
        let state = state.clone();
        let window = window.clone();
        move |_, _| {
            let dialog =
                FileChooserNative::new(None, Some(&window), FileChooserAction::Save, None, None);

            if let Some(ref file) = *state.open_file.borrow() {
                let file = gio::File::new_for_path(file);
                dialog.set_file(&file).ok();
            } else {
                dialog.set_current_name("Untitled.monti");
            }
            dialog.set_do_overwrite_confirmation(true);

            let filter = FileFilter::new();
            FileFilterExt::set_name(&filter, "Monotile file");
            filter.add_pattern("*.monti");
            dialog.add_filter(&filter);

            dialog.connect_response({
                let app = app.clone();
                let state = state.clone();
                move |dialog, resp| {
                    if resp == gtk::ResponseType::Accept.to_glib() {
                        let files = dialog.get_files();
                        if files.len() < 1 {
                            return;
                        }
                        if let Some(path) = files[0].get_path() {
                            let canvas = state.canvas.borrow();
                            match file_formats::save(&path, &canvas) {
                                Ok(_) => {
                                    state.open_file.replace(Some(path.clone()));
                                    app.activate_action("file_changed", None);
                                }
                                Err(err) => println!("Failed to save as: {}", err),
                            }
                        } else {
                            eprintln!("Failed to get path for file");
                        }
                    }
                }
            });
            dialog.run();
        }
    });

    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate({
        let app = app.clone();
        move |_, _| {
            app.quit();
        }
    });

    let file_changed = gio::SimpleAction::new("file_changed", None);
    file_changed.connect_activate({
        let state = state.clone();
        let window = window.clone();
        move |_, _| {
            window
                .get_window()
                .map(|window| window.invalidate_rect(None, true));
        }
    });

    app.add_action(&new);
    app.add_action(&open);
    app.add_action(&save);
    app.add_action(&saveas);
    app.add_action(&quit);
    app.add_action(&file_changed);
}

pub fn build(app: &gtk::Application) {
    build_menu(app);

    let state: Rc<State> = Rc::new(State {
        open_file: RefCell::new(None),
        canvas: RefCell::new(Canvas::new(32, 32)),
        canvas_cursor_position: RefCell::new(None),
        tileset: tileset::Tileset::new(),
        current_tile: RefCell::new(Tile {
            index: 0,
            fg: Color::rgb(1.0, 1.0, 1.0),
            bg: Color::rgb(0.0, 0.0, 0.0),
        }),
    });

    app.connect_open({
        let state = state.clone();
        move |app, files, _hint| {
            let path = files[0].get_path().expect("get_path failed");
            match file_formats::load(&path) {
                Ok(canvas) => {
                    state.canvas.replace(canvas);
                    state.open_file.replace(Some(path));
                    app.activate_action("file_changed", None);
                }
                Err(err) => {
                    println!("Opening file failed: {}", err);
                }
            }
        }
    });

    let window = gtk::ApplicationWindow::new(app);

    add_actions(app, &window, &state);

    window.set_title("Monotile");
    window.set_default_size(300, 300);

    let app_box = gtk::Box::new(Orientation::Vertical, 0);
    app_box.set_border_width(3);
    let main_area = gtk::Box::new(Orientation::Horizontal, 2);
    app_box.add(&main_area);
    let side_bar = gtk::Box::new(Orientation::Vertical, 2);

    let drawing_area = drawing_area::build(&state);
    main_area.add(&drawing_area);
    main_area.add(&side_bar);

    let tile_chooser = tile_chooser::build(&state);
    let color_chooser = color_chooser::build(&state);
    side_bar.add(&tile_chooser);
    side_bar.add(&color_chooser);

    window.add(&app_box);

    window.show_all();
}
