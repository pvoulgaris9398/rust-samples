mod custom_button;

use custom_button::CustomButton;
use glib::closure_local;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let button1 = CustomButton::new();
    let button2 = CustomButton::new();
    // button.set_margin_top(12);
    // button.set_margin_bottom(12);
    // button.set_margin_start(12);
    // button.set_margin_end(12);

    button1.connect_closure(
        "max-number-reached",
        false,
        closure_local!(move |_button: CustomButton, number: i32| {
            println!("The maximum number {} has been reached!", number);
        }),
    );

    button1
        .bind_property("number", &button2, "number")
        .transform_to(|_, number: i32| {
            let incremented_number = number + 1;
            Some(incremented_number.to_value())
        })
        .transform_from(|_, number: i32| {
            let decremented_number = number - 1;
            Some(decremented_number.to_value())
        })
        .bidirectional()
        .sync_create()
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    gtk_box.append(&button1);
    gtk_box.append(&button2);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present window
    window.present();
}
