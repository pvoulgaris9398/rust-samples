mod grid_cell;

use gtk::{
    gio,
    glib::{self, BoxedAnyObject},
    prelude::*,
};

use crate::grid_cell::{Entry, GridCell};

struct Row {
    col1: String,
    col2: String,
    col3: String,
    col4: String,
}

use std::cell::Ref;

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.columnview-example")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(320)
        .default_height(480)
        .application(application)
        .title("ColumnView Example")
        .build();

    let store = gio::ListStore::new::<BoxedAnyObject>();

    (0..10000).for_each(|i| {
        store.append(&BoxedAnyObject::new(Row {
            col1: format!("Security Id {i}"),
            col2: format!("Security Name {i}"),
            col3: format!("Asset Class {i}"),
            col4: format!("Last Price {i}"),
        }))
    });
    let sel = gtk::SingleSelection::new(Some(store));
    let columnview = gtk::ColumnView::new(Some(sel));

    let col1factory = gtk::SignalListItemFactory::new();
    let col2factory = gtk::SignalListItemFactory::new();
    let col3factory = gtk::SignalListItemFactory::new();
    let col4factory = gtk::SignalListItemFactory::new();

    col1factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::default();
        item.set_child(Some(&row));
    });

    col1factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.col1.to_string(),
        };
        child.set_entry(&ent);
    });

    col2factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::default();
        item.set_child(Some(&row));
    });

    col2factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.col2.to_string(),
        };
        child.set_entry(&ent);
    });

    col3factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::default();
        item.set_child(Some(&row));
    });

    col3factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.col3.to_string(),
        };
        child.set_entry(&ent);
    });

    col4factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::default();
        item.set_child(Some(&row));
    });

    col4factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.col4.to_string(),
        };
        child.set_entry(&ent);
    });

    let col1 = gtk::ColumnViewColumn::new(Some("Column 1"), Some(col1factory));
    let col2 = gtk::ColumnViewColumn::new(Some("Column 2"), Some(col2factory));
    let col3 = gtk::ColumnViewColumn::new(Some("Column 3"), Some(col3factory));
    let col4 = gtk::ColumnViewColumn::new(Some("Column 4"), Some(col4factory));

    columnview.append_column(&col1);
    columnview.append_column(&col2);
    columnview.append_column(&col3);
    columnview.append_column(&col4);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .build();

    scrolled_window.set_child(Some(&columnview));

    window.set_child(Some(&scrolled_window));
    window.present();
}
