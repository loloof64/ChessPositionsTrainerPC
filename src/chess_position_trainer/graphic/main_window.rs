use gtk::prelude::*;
use gtk::{Window, WindowType};
use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;


use chess_position_trainer::graphic::ChessBoard;

pub struct MainWindow
{
    window: Window,
}

impl MainWindow
{
    pub fn new() -> MainWindow
    {
        let mut main_window = MainWindow{window: Window::new(WindowType::Toplevel)};
        main_window.initialize();
        main_window
    }

    pub fn show(&self)
    {
        self.window.show_all();
    }

    fn initialize(&mut self)
    {
        let cells_size = 50u32;
        self.set_size_and_title(cells_size);
        self.set_icon();

        let chessboard = ChessBoard::new_from_fen(
            cells_size, 
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
        ).expect("Failed to intialize the chessboard !");

        self.window.add(chessboard.get_drawing_area());
        
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }

    fn set_size_and_title(&mut self, cells_size: u32){
        self.window.set_title("Chess Position Trainer");
        let window_size = cells_size as i32 * 9;
        self.window.set_default_size(window_size, window_size);
    }

    fn set_icon(&mut self){
        let icon_stream = MemoryInputStream::new_from_bytes(
            &Bytes::from_static(include_bytes!("../../resources/Chess_ql.png"))
        );
        let icon_pixbuf = Pixbuf::new_from_stream(&icon_stream, None);
        let icon = match icon_pixbuf {
            Ok(icon) => icon,
            Err(e) => {
                println!("Failed to get icon ! ({})", e);
                return;
            }
        };
        self.window.set_icon(&icon);
    }
}