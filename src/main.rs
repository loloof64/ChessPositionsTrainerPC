extern crate gtk;

mod chess_position_trainer;
use chess_position_trainer::graphic::{MainWindow};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let main_window = MainWindow::new();
    main_window.show();

    gtk::main();
}