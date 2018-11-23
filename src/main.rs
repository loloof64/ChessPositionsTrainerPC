extern crate shakmaty;
extern crate gtk;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate cairo;
extern crate gdk_pixbuf;

mod chess_position_trainer;
use chess_position_trainer::graphic::main_window::{MainWindow};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let main_window = MainWindow::new();
    main_window.show();

    gtk::main();
}