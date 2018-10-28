extern crate gtk;

use gtk::prelude::*;

use gtk::{Button, Window, WindowType};

struct MainWindow
{
    window: Window,
}

impl MainWindow
{
    fn new() -> MainWindow
    {
        let main_window = MainWindow{window: Window::new(WindowType::Toplevel)};
        main_window.initialize();
        main_window
    }

    fn show(&self)
    {
        self.window.show_all();
    }

    fn initialize(&self)
    {
        self.window.set_title("Chess Position Trainer");
        self.window.set_default_size(350, 70);
        let button = Button::new_with_label("Click me!");
        self.window.add(&button);
        self.window.show_all();

        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        button.connect_clicked(|_| {
            println!("Clicked!");
        });
    }
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let main_window = MainWindow::new();
    main_window.show();

    gtk::main();
}