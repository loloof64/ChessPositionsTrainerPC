use gtk::prelude::*;
use gtk::{Window, WindowType, Button, Image, Box as GtkBox, Orientation};
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
        self.set_size_and_title();
        self.set_icon();

        let chessboard = ChessBoard::new_from_default()
            .expect("Failed to intialize the chessboard !");

        let reverse_board_button = Button::new();
        let reverse_board_button_image = Image::new_from_pixbuf(
            &load_image(
                include_bytes!("../../resources/UpDown.png"),
                20,
            ).expect("Could not find UpDown image !")
        );
        reverse_board_button.set_image(&reverse_board_button_image);

        reverse_board_button.connect_clicked({
            let chessboard = chessboard.clone();
            move |_button|{
                chessboard.borrow_mut().reverse();
            }
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

    fn set_size_and_title(&mut self){
        self.window.set_title("Chess Position Trainer");
        let window_width = 50i32 * 9;
        self.window.set_default_size(window_width, window_width + 65);
    }

    fn set_icon(&mut self){
        let icon_pixbuf = load_image(
                include_bytes!("../../resources/Chess_ql.png"),
                60,
            ).expect("Could not find UpDown image !");
        self.window.set_icon(&icon_pixbuf);
    }
}