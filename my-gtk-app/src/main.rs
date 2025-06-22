#![allow(unused, dead_code)]

use glib::clone;
use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};
const APP_ID: &str = "org.gtk_rs.HelloWorld1";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = std::rc::Rc::new(std::cell::Cell::new(0));

    button_increase.connect_clicked(clone!(
        #[weak]
        number,
        #[weak]
        button_decrease,
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
        }
    ));
    button_decrease.connect_clicked(clone!(
        #[weak]
        button_increase,
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
        }
    ));

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Initial rust-GTK Sample App")
        .child(&gtk_box)
        .build();

    window.present();
}
