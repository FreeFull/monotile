use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use gio;
use gio::{
    ActionGroupExt, ActionMapExt, ApplicationExt, ApplicationExtManual, FileExt, MenuExt,
    SimpleActionExt,
};
use gtk;
use gtk::{
    ContainerExt, FileChooserAction, FileChooserExt, FileChooserNative, GtkApplicationExt,
    GtkWindowExt, NativeDialogExt, WidgetExt,
};

mod canvas;
use self::canvas::Canvas;
mod drawing_area;

#[derive(Clone, Debug)]
pub struct State {
    pub open_file: RefCell<Option<PathBuf>>,
    pub canvas: RefCell<Canvas>,
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
            let app = app.clone();
            dialog.connect_response(move |dialog, _resp| {
                if let Some(filename) = dialog.get_filename() {
                    app.open(&[gio::File::new_for_path(filename)], "");
                }
            });
            dialog.run();
        }
    });

    let save = gio::SimpleAction::new("save", None);
    save.connect_activate({
        move |_, _| {
            println!("save");
        }
    });
    let saveas = gio::SimpleAction::new("saveas", None);
    saveas.connect_activate({
        move |_, _| {
            println!("save as");
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
        move |_, _| {
            println!("{:?}", state.open_file);
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
    });

    app.connect_open({
        let state = state.clone();
        move |app, files, _hint| {
            let path = files[0].get_path().expect("get_path failed");
            state.open_file.replace(Some(path));
            app.activate_action("file_changed", None);
        }
    });

    let window = gtk::ApplicationWindow::new(app);

    add_actions(app, &window, &state);

    window.set_title("Monotile");
    window.set_default_size(300, 300);

    let drawing_area = drawing_area::build(&state);
    window.add(&drawing_area);

    window.show_all();
}
