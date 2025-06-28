use std::thread;
use std::time::Duration;

use gtk::prelude::*;
use gtk::{self, gio, glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.MainEventLoop";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |_| {
        gio::spawn_blocking(move || {
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
        });
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Initial rust-GTK Sample App")
        .child(&button)
        .build();

    window.present();
}
