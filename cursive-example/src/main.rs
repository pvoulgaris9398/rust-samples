// https://docs.rs/cursive/latest/cursive/index.html

use cursive::views::{Dialog, EditView, ListView, TextView};
use cursive::{Cursive, CursiveExt};

fn main() {
    let mut w = Cursive::new();

    w.add_layer(
        Dialog::new().title("Cursive GUI Example").content(
            ListView::new()
                .child("Item 1", TextView::new("Content of TextView"))
                .child("Item 2", EditView::new().content("Editable view"))
                //.add_delimiter()
                .child("Item 3", TextView::new("another item")),
        ),
    );

    w.run();
}
