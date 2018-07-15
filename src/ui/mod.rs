use std::cell::RefCell;
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
mod state;
mod tileset;
pub use self::state::State;

fn build_menu(app: &gtk::Application) {
    let menu = gio::Menu::new();
    let file = gio::Menu::new();
    menu.append_submenu("File", &file);
    file.append("New", "app.new");
    file.append("Open", "app.open");
    file.append("Save", "app.save");
    file.append("Save as", "app.saveas");

    file.append("Quit", "app.quit");
    let edit = gio::Menu::new();
    menu.append_submenu("Edit", &edit);
    edit.append("Undo", "app.undo");
    edit.append("Redo", "app.redo");
    edit.append("Cut", "app.cut");
    edit.append("Copy", "app.copy");
    edit.append("Paste", "app.paste");

    let help = gio::Menu::new();
    menu.append_submenu("Help", &help);
    help.append("Monotile Help", "app.help");
    help.append("About Monotile", "app.about");

    app.set_menubar(&menu);
}

fn add_actions(state: &Rc<State>) {
    let new = gio::SimpleAction::new("new", None);
    new.connect_activate({
        let state = state.clone();
        move |_, _| {
            state.open_file.replace(None);
            state.canvas.replace(Canvas::default());
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
        move |_, _| {
            let open_file = state.open_file.borrow().clone();
            match open_file {
                Some(ref path) => {
                    let canvas = state.canvas.borrow();
                    match file_formats::save(&path, &canvas) {
                        Ok(_) => {}
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
            FileFilterExt::set_name(&filter, "Monotile file");
            filter.add_pattern("*.monti");
            dialog.add_filter(&filter);

            dialog.connect_response({
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
        }
    });

    state.app.add_action(&new);
    state.app.add_action(&open);
    state.app.add_action(&save);
    state.app.add_action(&saveas);
    state.app.add_action(&quit);
    state.app.add_action(&file_changed);
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

pub fn build(app: &gtk::Application) {
    build_menu(app);

    let window = gtk::ApplicationWindow::new(app);

    let state: Rc<State> = Rc::new(State {
        app: app.clone(),
        window: window.clone(),
        open_file: RefCell::new(None),
        canvas: RefCell::new(Canvas::new(32, 32)),
        canvas_cursor_position: RefCell::new(None),
        tileset: tileset::Tileset::new(),
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

    window.show_all();
}
