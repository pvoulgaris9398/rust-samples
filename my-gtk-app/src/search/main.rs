use gtk::{
    glib::{self, clone},
    prelude::*,
};

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("com.github.pete.search_bar")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);

    window.set_default_size(400, 400);

    window.set_title(Some("Search Bar"));

    let header_bar = gtk::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let search_button = gtk::ToggleButton::new();
    search_button.set_icon_name("system-search-symbolic");
    header_bar.pack_end(&search_button);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    window.set_child(Some(&container));

    let search_bar = gtk::SearchBar::builder()
        .valign(gtk::Align::Start)
        .key_capture_widget(&window)
        .build();

    container.append(&search_bar);

    search_button
        .bind_property("active", &search_bar, "search-mode-enabled")
        .sync_create()
        .bidirectional()
        .build();

    let entry = gtk::SearchEntry::new();
    entry.set_hexpand(true);
    search_bar.set_child(Some(&entry));

    let label = gtk::Label::builder()
        .label("Type to start...")
        .vexpand(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .css_classes(["large-title"])
        .build();

    container.append(&label);

    entry.connect_stop_search(clone!(
        #[weak]
        search_button,
        move |_| {
            search_button.set_active(false);
        }
    ));

    entry.connect_search_changed(clone!(
        #[weak]
        label,
        move |entry| {
            if entry.text() != "" {
                label.set_text(&entry.text());
            } else {
                label.set_text("Type to start search");
            }
        }
    ));

    window.present();
}
