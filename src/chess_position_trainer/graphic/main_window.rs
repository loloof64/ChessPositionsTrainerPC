use gtk::prelude::*;
use gtk::{Window, WindowType};

use chess_position_trainer::graphic::ChessBoard;

pub struct MainWindow
{
    window: Window,
}

impl MainWindow
{
    pub fn new() -> MainWindow
    {
        let main_window = MainWindow{window: Window::new(WindowType::Toplevel)};
        main_window.initialize();
        main_window
    }

    pub fn show(&self)
    {
        self.window.show_all();
    }

    fn initialize(&self)
    {
        self.window.set_title("Chess Position Trainer");
        self.window.set_default_size(350, 70);
        let chessboard = ChessBoard::new(50);

        self.window.add(chessboard.get_drawing_area());
        
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }
}