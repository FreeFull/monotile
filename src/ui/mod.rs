use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gdk::WindowExt;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{FileChooserAction, FileChooserNative, FileFilter, Orientation};

mod canvas;
use self::canvas::{Canvas, Color, Tile};
mod components;
use self::components::*;
mod file_formats;
mod state;
mod tileset;
use self::state::CanvasSurface;
pub use self::state::State;

fn build_menu(app: &gtk::Application) {
    let menu = gio::Menu::new();
    let file = gio::Menu::new();
    menu.append_submenu(Some("File"), &file);
    file.append(Some("New"), Some("app.new"));
    file.append(Some("Open"), Some("app.open"));
    file.append(Some("Save"), Some("app.save"));
    file.append(Some("Save as"), Some("app.saveas"));
    file.append(Some("Quit"), Some("app.quit"));
    let edit = gio::Menu::new();
    menu.append_submenu(Some("Edit"), &edit);
    edit.append(Some("Undo"), Some("app.undo"));
    edit.append(Some("Redo"), Some("app.redo"));
    edit.append(Some("Cut"), Some("app.cut"));
    edit.append(Some("Copy"), Some("app.copy"));
    edit.append(Some("Paste"), Some("app.paste"));

    let help = gio::Menu::new();
    menu.append_submenu(Some("Help"), &help);
    help.append(Some("Monotile Help"), Some("app.help"));
    help.append(Some("About Monotile"), Some("app.about"));

    app.set_menubar(Some(&menu));
}

fn add_actions(state: &Rc<State>) {
    let new = gio::SimpleAction::new("new", None);
    new.connect_activate({
        let state = state.clone();
        move |_, _| {
            state.open_file.replace(None);
            state
                .canvas_surface
                .borrow_mut()
                .set_canvas(Canvas::default());
            state.current_tile.replace(Tile {
                index: 0,
                fg: Color::rgb(255, 255, 255),
                bg: Color::rgb(0, 0, 0),
            });
            state.app.activate_action("file_changed", None);
        }
    });

    let open = gio::SimpleAction::new("open", None);
    open.connect_activate({
        let app = state.app.clone();
        let window = state.window.clone();
        move |_, _| {
            let dialog =
                FileChooserNative::new(None, Some(&window), FileChooserAction::Open, None, None);
            dialog.set_show_hidden(false);

            let filter = FileFilter::new();
            filter.set_name(Some("Monotile file"));
            filter.add_pattern("*.monti");
            dialog.add_filter(&filter);

            let app = app.clone();
            dialog.connect_response(move |dialog, resp| {
                if resp == gtk::ResponseType::Accept {
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
        move |_, _| {
            let open_file = state.open_file.borrow().clone();
            match open_file {
                Some(ref path) => {
                    let canvas = &state.canvas_surface.borrow().canvas;
                    match file_formats::save(&path, &canvas) {
                        Ok(_) => {
                            state.modified.set(false);
                            set_title(&state);
                        }
                        Err(err) => println!("save failed: {}", err),
                    }
                }
                None => {
                    state.app.activate_action("saveas", None);
                }
            }
        }
    });
    let saveas = gio::SimpleAction::new("saveas", None);
    saveas.connect_activate({
        let state = state.clone();
        move |_, _| {
            let dialog = FileChooserNative::new(
                None,
                Some(&state.window),
                FileChooserAction::Save,
                None,
                None,
            );

            if let Some(ref file) = *state.open_file.borrow() {
                let file = gio::File::new_for_path(file);
                dialog.set_file(&file).ok();
            } else {
                dialog.set_current_name("Untitled.monti");
            }
            dialog.set_do_overwrite_confirmation(true);

            let filter = FileFilter::new();
            filter.set_name(Some("Monotile file"));
            filter.add_pattern("*.monti");
            dialog.add_filter(&filter);

            dialog.connect_response({
                let state = state.clone();
                move |dialog, resp| {
                    if resp == gtk::ResponseType::Accept {
                        let files = dialog.get_files();
                        if files.len() < 1 {
                            return;
                        }
                        if let Some(path) = files[0].get_path() {
                            let canvas = &state.canvas_surface.borrow().canvas;
                            match file_formats::save(&path, &canvas) {
                                Ok(_) => {
                                    state.open_file.replace(Some(path.clone()));
                                    state.app.activate_action("file_changed", None);
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
        let app = state.app.clone();
        move |_, _| {
            app.quit();
        }
    });

    let file_changed = gio::SimpleAction::new("file_changed", None);
    file_changed.connect_activate({
        let state = state.clone();
        move |_, _| {
            state
                .window
                .get_window()
                .map(|window| window.invalidate_rect(None, true));
            state.modified.set(false);
            set_title(&state);
        }
    });

    let modified = gio::SimpleAction::new("modified", None);
    modified.connect_activate({
        let state = state.clone();
        move |_, _| {
            state.modified.set(true);
            set_title(&state);
        }
    });

    state.app.add_action(&new);
    state.app.add_action(&open);
    state.app.add_action(&save);
    state.app.add_action(&saveas);
    state.app.add_action(&quit);
    state.app.add_action(&file_changed);
    state.app.add_action(&modified);
}

fn add_accelerators(state: &Rc<State>) {
    let app = &state.app;
    app.set_accels_for_action("app.new", &["<Primary>n"]);
    app.set_accels_for_action("app.open", &["<Primary>o"]);
    app.set_accels_for_action("app.save", &["<Primary>s"]);
    app.set_accels_for_action("app.saveas", &["<Primary><Shift>s"]);
    app.set_accels_for_action("app.quit", &["<Primary>q"]);

    app.set_accels_for_action("app.undo", &["<Primary>z"]);
    app.set_accels_for_action("app.redo", &["<Primary><Shift>z", "<Primary>r"]);
    app.set_accels_for_action("app.cut", &["<Primary>x"]);
    app.set_accels_for_action("app.copy", &["<Primary>c"]);
    app.set_accels_for_action("app.paste", &["<Primary>v"]);

    app.set_accels_for_action("app.help", &["F1"]);
}

fn set_title(state: &Rc<State>) {
    let mut title = String::from("Monotile - ");
    if state.modified.get() {
        title.push('*');
    }
    title.push('[');
    match *state.open_file.borrow() {
        Some(ref file) => {
            match file.file_name() {
                Some(name) => {
                    title.push_str(&name.to_string_lossy());
                }
                None => {
                    eprintln!("Warning: file path ends in ..");
                }
            }
        }
        None => {
            title.push_str("Untitled");
        }
    }
    title.push(']');
    state.window.set_title(&title);
}

pub fn build(app: &gtk::Application) {
    build_menu(app);

    let window = gtk::ApplicationWindow::new(app);

    let state: Rc<State> = Rc::new(State {
        app: app.clone(),
        window: window.clone(),
        open_file: RefCell::new(None),
        modified: Cell::new(false),
        canvas_surface: RefCell::new(CanvasSurface::new(
            Canvas::new(32, 32),
            tileset::Tileset::new(),
        )),
        canvas_cursor_position: RefCell::new(None),
        current_tile: RefCell::new(Tile {
            index: 0,
            fg: Color::rgb(255, 255, 255),
            bg: Color::rgb(0, 0, 0),
        }),
        current_tool: RefCell::new(state::Tool::Draw),
    });

    app.connect_open({
        let state = state.clone();
        move |app, files, _hint| {
            let path = files[0].get_path().expect("get_path failed");
            match file_formats::load(&path) {
                Ok(canvas) => {
                    state.canvas_surface.borrow_mut().set_canvas(canvas);
                    state.open_file.replace(Some(path));
                    app.activate_action("file_changed", None);
                }
                Err(err) => {
                    println!("Opening file failed: {}", err);
                }
            }
        }
    });

    add_actions(&state);
    add_accelerators(&state);

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
    let tool_chooser = tool_chooser::build(&state);
    side_bar.add(&tile_chooser);
    side_bar.add(&color_chooser);
    side_bar.add(&tool_chooser);

    window.add(&app_box);

    set_title(&state);

    window.show_all();
}
