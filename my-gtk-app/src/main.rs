use gtk::prelude::*;
use gtk::{glib,Application, ApplicationWindow};
const APP_ID: &str = "org.gtk_rs.HelloWorld1";
fn main()->glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application)
{
    let window = ApplicationWindow::builder()
    .application(app)
    //.width(600)
    //.height(300)
    .title("Initial rust-GTK Sample App")
    .build();
    
    window.present();
}
