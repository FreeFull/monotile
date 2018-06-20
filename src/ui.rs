use gio;
use gio::{ActionMapExt, ApplicationExt, ApplicationExtManual, MenuExt, SimpleActionExt};
use gtk;
use gtk::{
    FileChooserAction, FileChooserExt, FileChooserNative, GtkApplicationExt, GtkWindowExt,
    NativeDialogExt, WidgetExt,
};

fn build_menu(app: &gtk::Application) {
    let menu = gio::Menu::new();
    menu.append("Open", "app.open");
    menu.append("Save", "app.save");
    menu.append("Save as", "app.saveas");

    menu.append("Quit", "app.quit");

    app.set_app_menu(&menu);
}

fn add_actions(app: &gtk::Application, window: &gtk::ApplicationWindow) {
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
                    app.open(&[gio::File::new_for_path(filename)],"");
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

    app.add_action(&open);
    app.add_action(&save);
    app.add_action(&saveas);
    app.add_action(&quit);
}

pub fn build(app: &gtk::Application) {
    build_menu(app);

    app.connect_open(|_app, files, hint| {
        println!("{:?} {:?}", files, hint);
    });

    let window = gtk::ApplicationWindow::new(app);

    add_actions(app, &window);

    window.set_title("Monotile");
    window.set_default_size(300, 300);

    window.show_all();
}
