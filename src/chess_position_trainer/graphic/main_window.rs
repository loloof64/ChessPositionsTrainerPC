use gtk::prelude::*;
use gtk::{Window, WindowType, Button, Image, Box as GtkBox, Orientation};
use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;
use chess_position_trainer::graphic::{ChessBoard, load_image};

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

        let reverse_board_button = Button::new();
        let reverse_board_button_image = Image::new_from_pixbuf(
            &load_image(
                include_bytes!("../../resources/UpDown.png"),
                20,
            ).expect("Could not find UpDown image !")
        );
        reverse_board_button.set_image(&reverse_board_button_image);

        let click_handler_chessboard = chessboard.clone();
        reverse_board_button.connect_clicked(move |_button|{
            click_handler_chessboard.borrow_mut().reverse();
        });

        let buttons_hbox = GtkBox::new(
            Orientation::Horizontal,
            20,
        );
        buttons_hbox.pack_start(
            &reverse_board_button,
            true,
            false,
            10,
        );

        let window_vbox = GtkBox::new(
            Orientation::Vertical,
            0,
        );
        window_vbox.pack_start(
            &buttons_hbox,
            false,
            false,
            10,
        );
        window_vbox.pack_start(
            chessboard.borrow().get_drawing_area(),
            true,
            true,
            0,
        );

        self.window.add(&window_vbox);
        
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }

    fn set_size_and_title(&mut self, cells_size: u32){
        self.window.set_title("Chess Position Trainer");
        let window_width = cells_size as i32 * 9;
        self.window.set_default_size(window_width, window_width + 65);
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