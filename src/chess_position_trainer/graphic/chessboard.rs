use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;
use gdk::prelude::*;
use gdk::{EventMask, EventType};
use gtk::{DrawingArea, Dialog, Orientation, Box as GBox, Button, Image};
use gdk_pixbuf::Pixbuf;
use cairo::Context;
use cairo::enums::{FontSlant, FontWeight};
use shakmaty::{Piece, Role, Chess};
use super::load_image;
use super::super::logic::chessgame::ChessGame;

#[derive(Clone)]
pub struct ChessBoard
{
    drawing_area: DrawingArea,
    reversed: bool,
    logic: RefCell<ChessGame>,
    cells_size: u32,
    moved_piece: RefCell<Option<MovedPiece>>,
    pieces_images: HashMap<char, Pixbuf>,
}

#[derive(Clone, Debug)]
struct MovedPiece
{
    piece_type: Piece,
    coords_x: f64,
    coords_y: f64,
    start_file: u8,
    start_rank: u8
}

enum PromotionType
{
    QUEEN = 1,
    ROOK = 2,
    BISHOP = 4,
    KNIGHT = 8
}

impl MovedPiece {
    fn translate_to(&mut self, x: f64, y: f64)
    {
        self.coords_x = x;
        self.coords_y = y;
    }
}

impl ChessBoard
{
    pub fn new_from_default() -> Result<Rc<RefCell<ChessBoard>>, String>
    {
        ChessBoard::new(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        )
    }

    pub fn new(initial_position: &str) -> Result<Rc<RefCell<ChessBoard>>, String>
    {
        ChessBoard::get_chessboard(
            initial_position,
        )
    }

    pub fn reverse(&mut self) 
    {
        self.reversed = ! self.reversed;
        self.drawing_area.queue_draw();
    }

    pub fn get_drawing_area(&self) -> &DrawingArea
    {
        &self.drawing_area
    }
    
    fn load_pieces_images(size: u32) -> HashMap<char, Pixbuf> {
        let mut images = HashMap::new();

        let params: Vec<(char, &'static [u8], &str)> = vec![
            ('P', include_bytes!("../../resources/Chess_pl.png"), "Failed to get white pawn image !"),
            ('N', include_bytes!("../../resources/Chess_nl.png"), "Failed to get white knight image !"),
            ('B', include_bytes!("../../resources/Chess_bl.png"), "Failed to get white bishop image !"),
            ('R', include_bytes!("../../resources/Chess_rl.png"), "Failed to get white rook image !"),
            ('Q', include_bytes!("../../resources/Chess_ql.png"), "Failed to get white queen image !"),
            ('K', include_bytes!("../../resources/Chess_kl.png"), "Failed to get white king image !"),

            ('p', include_bytes!("../../resources/Chess_pd.png"), "Failed to get black pawn image !"),
            ('n', include_bytes!("../../resources/Chess_nd.png"), "Failed to get black knight image !"),
            ('b', include_bytes!("../../resources/Chess_bd.png"), "Failed to get black bishop image !"),
            ('r', include_bytes!("../../resources/Chess_rd.png"), "Failed to get black rook image !"),
            ('q', include_bytes!("../../resources/Chess_qd.png"), "Failed to get black queen image !"),
            ('k', include_bytes!("../../resources/Chess_kd.png"), "Failed to get black king image !"),
        ];

        params.iter().for_each(|(piece_code, image_binary, error_string)| {
            images.insert(
                *piece_code,
                load_image(
                    image_binary,
                    size as i32
                ).expect(error_string)
            );

        });

        images
    }

    fn get_chessboard(initial_position: &str) -> Result<Rc<RefCell<ChessBoard>>, String>
    {
        let drawing_area = DrawingArea::new();
        drawing_area.add_events((
            EventMask::BUTTON1_MOTION_MASK.bits() |
            EventMask::BUTTON_PRESS_MASK.bits() |
            EventMask::BUTTON_RELEASE_MASK.bits()
        ) as i32);

        let logic = ChessGame::new_from_fen(initial_position);

        match logic {
            Some(game_logic) => {
                let pieces_images = ChessBoard::load_pieces_images((50f64 * 0.8) as u32);

                let chess_board = ChessBoard {
                    drawing_area,
                    reversed: false,
                    logic: RefCell::new(game_logic),
                    cells_size: 50u32,
                    moved_piece: RefCell::new(None),
                    pieces_images,
                };

                let chess_board_ref = Rc::new(RefCell::new(chess_board));

                chess_board_ref.borrow().drawing_area.connect_draw({
                    let chess_board_ref = chess_board_ref.clone();
                    move |_drawing_area, cr|{
                        chess_board_ref.borrow().paint(cr);
                        Inhibit(false)
                    }
                });

                chess_board_ref.borrow().drawing_area.connect_event({
                    let chess_board_ref = chess_board_ref.clone();
                    move |_self, event| {
                        let coords = event.get_coords().expect("Failed to get mouse coordinates !");
                        
                        match event.get_event_type() {
                            EventType::ButtonPress => chess_board_ref.borrow().handle_mouse_pressed(coords),
                            EventType::ButtonRelease => chess_board_ref.borrow().handle_mouse_released(coords),
                            EventType::MotionNotify => chess_board_ref.borrow().handle_mouse_moved(coords),
                        _ => {} 
                        }
                        Inhibit(false)
                    }
                });

                Ok(chess_board_ref)
            },
            None => Err(format!("Bad FEN {} !", initial_position))
        }
    }

    fn handle_mouse_pressed(&self, coords: (f64, f64)){
        let cells_size = self.cells_size as f64;
        let mut cell_coords = (
            ((coords.0 - (cells_size * 0.5)) / cells_size) as i32,
            7 - (((coords.1 - (cells_size * 0.5)) / cells_size) as i32),
        );
        if self.reversed {
            cell_coords = (7-cell_coords.0, 7-cell_coords.1);
        }

        let (coords_x, coords_y) = coords;
        let moved_piece = self.logic.borrow().piece_at_cell(cell_coords.0 as i8, cell_coords.1 as i8);
        if let Some(piece_type) = moved_piece {
            self.moved_piece.replace(Some(MovedPiece{
                coords_x,
                coords_y,
                piece_type,
                start_file: cell_coords.0 as u8,
                start_rank: cell_coords.1 as u8,
            }));

            self.drawing_area.queue_draw();
        }
    }

    fn handle_mouse_released(&self, coords: (f64, f64)){
        let cells_size = self.cells_size as f64;
        let mut cell_coords = (
            ((coords.0 - (cells_size * 0.5)) / cells_size) as u8,
            7 - (((coords.1 - (cells_size * 0.5)) / cells_size) as u8),
        );
        if self.reversed {
            cell_coords = ((7-cell_coords.0) as u8, (7-cell_coords.1) as u8);
        }

        if let Some(moved_piece) = self.moved_piece.borrow().clone() {
            let start_cell = (moved_piece.start_file, moved_piece.start_rank);
            let end_cell = cell_coords;

            if self.logic.borrow().is_legal_move::<Chess>(start_cell, end_cell) {
                if self.logic.borrow().is_promotion_move(start_cell, end_cell) {
                    let selected_role = self.open_promotion_selector();
                    self.logic.borrow_mut().do_move::<Chess>(start_cell, end_cell, Some(selected_role));
                    self.drawing_area.queue_draw();
                }
                else {
                    self.logic.borrow_mut().do_move::<Chess>(start_cell, end_cell, None);
                    self.drawing_area.queue_draw();
                }
            }
        }

        self.moved_piece.replace(None);
        self.drawing_area.queue_draw();
    }

    fn handle_mouse_moved(&self, coords: (f64, f64)) {
        match *self.moved_piece.borrow_mut() {
            Some(ref mut moved_piece) => {
                moved_piece.translate_to(coords.0, coords.1);
                self.drawing_area.queue_draw();
            }
            None => {}
        }
    }

    fn open_promotion_selector(&self) -> Role {
        let dialog = Dialog::new();
        dialog.set_title("Select your promotion piece");
        dialog.set_modal(true);

        let dialog_ref = Rc::new(dialog);
        let is_white_turn = self.logic.borrow().is_white_turn();

        let queen_image = if is_white_turn {
            self.pieces_images.get(&'Q').expect("Failed to get white queen image")
        } else {
            self.pieces_images.get(&'q').expect("Failed to get black queen image")
        };
        let queen_image = Image::new_from_pixbuf(queen_image);

        let queen_button = Button::new();
        queen_button.set_image(&queen_image);
        queen_button.connect_clicked({
            let dialog_ref = dialog_ref.clone();
            move |_button| {
                dialog_ref.response(PromotionType::QUEEN as i32);
                dialog_ref.close();
            }
        });

        let rook_image = if is_white_turn {
            self.pieces_images.get(&'R').expect("Failed to get white rook image")
        } else {
            self.pieces_images.get(&'r').expect("Failed to get black rook image")
        };
        let rook_image = Image::new_from_pixbuf(rook_image);

        let rook_button = Button::new();
        rook_button.set_image(&rook_image);
        rook_button.connect_clicked({
            let dialog_ref = dialog_ref.clone();
            move |_button| {
                dialog_ref.response(PromotionType::ROOK as i32);
                dialog_ref.close();
            }
        });

        let bishop_image = if is_white_turn {
            self.pieces_images.get(&'B').expect("Failed to get white bishop image")
        } else {
            self.pieces_images.get(&'b').expect("Failed to get black bishop image")
        };
        let bishop_image = Image::new_from_pixbuf(bishop_image);

        let bishop_button = Button::new();
        bishop_button.set_image(&bishop_image);
        bishop_button.connect_clicked({
            let dialog_ref = dialog_ref.clone();
            move |_button| {
                dialog_ref.response(PromotionType::BISHOP as i32);
                dialog_ref.close();
            }
        });

        let knight_image = if is_white_turn {
            self.pieces_images.get(&'N').expect("Failed to get white knight image")
        } else {
            self.pieces_images.get(&'n').expect("Failed to get black knight image")
        };
        let knight_image = Image::new_from_pixbuf(knight_image);

        let knight_button = Button::new();
        knight_button.set_image(&knight_image);
        knight_button.connect_clicked({
            let dialog_ref = dialog_ref.clone();
            move |_button| {
                dialog_ref.response(PromotionType::KNIGHT as i32);
                dialog_ref.close();
            }
        });

        let buttons_box = GBox::new(Orientation::Horizontal, 10);
        buttons_box.pack_start(
            &queen_button,
            true,
            true,
            0
        );
        buttons_box.pack_start(
            &rook_button,
            true,
            true,
            0
        );
        buttons_box.pack_start(
            &bishop_button,
            true,
            true,
            0
        );
        buttons_box.pack_start(
            &knight_button,
            true,
            true,
            0
        );

        dialog_ref.get_content_area().pack_start(
            &buttons_box,
            true,
            true,
            10
        );

        dialog_ref.show_all();

        let response = dialog_ref.run();
        if response == PromotionType::QUEEN as i32 { Role::Queen }
                        else if response == PromotionType::ROOK as i32 { Role::Rook }
                        else if response == PromotionType::BISHOP as i32 { Role::Bishop }
                        else if response == PromotionType::KNIGHT as i32 { Role::Knight }
                        else { Role::Queen }


    }

    fn paint(&self, cr: &Context){
        self.draw_background(cr);
        self.draw_cells(cr);
        self.draw_pieces(cr);
        self.draw_moved_piece(cr);
        self.draw_coordinates(cr);
        self.draw_player_turn(cr);
    }

    fn draw_background(&self, cr: &Context)
    {
        let green_color = [60.0/255.0, 204.0/255.0, 100.0/255.0];
        cr.set_source_rgb(
            green_color[0],
            green_color[1],
            green_color[2],
        );
        cr.paint();
    }

    fn draw_cells(&self, cr: &Context)
    {
        (0..8).for_each(|rank| {
            (0..8).for_each(|file| {
                let white_cell_color = [255.0/255.0, 255.0/255.0, 179.0/255.0];
                let black_cell_color = [153.0/255.0, 102.0/255.0, 51.0/255.0];

                let is_white_cell = (file + rank) % 2 == 0;
                let cell_color = if is_white_cell {white_cell_color} else {black_cell_color};

                let rect_x = (self.cells_size as f64) * (0.5 + (file as f64));
                let rect_y = (self.cells_size as f64) * (0.5 + (rank as f64));
                let rect_size = self.cells_size as f64;

                cr.rectangle(
                    rect_x,
                    rect_y,
                    rect_size,
                    rect_size,
                );
                cr.set_source_rgb(
                    cell_color[0],
                    cell_color[1],
                    cell_color[2],
                );
                cr.fill();
            });
        });
    }

    fn draw_pieces(&self, cr: &Context)
    {
        (0..64).for_each(|index| {

            let file = index % 8;
            let rank = index / 8;
                
            let real_file = (if self.reversed { 7-file } else { file }) as u8;
            let real_rank = (if self.reversed { 7-rank } else { rank }) as u8;

            
            if let Some(piece) = self.logic.borrow().piece_at_cell(real_file as i8, real_rank as i8) {
                let moved_piece = self.moved_piece.borrow().clone();
                let not_moved_piece = 
                    if moved_piece.is_none() { true }
                    else {
                        let moved_piece = moved_piece.expect("Failed to get moved piece !");
                        moved_piece.start_file != real_file || moved_piece.start_rank != real_rank
                    };

                if not_moved_piece {
                        let image = self.pieces_images.get(&piece.char()).expect("Failed to get piece image !");
                        let location_x = (self.cells_size as f64) * (file as f64 + 0.5 + 0.1);
                        let location_y = (self.cells_size as f64) * ((7.0-rank as f64) + 0.5 + 0.1);
                        cr.set_source_pixbuf(
                            &image,
                            location_x,
                            location_y
                        );
                        cr.paint();   
                }
            }
        });
    }

    fn draw_moved_piece(&self, cr: &Context)
    {
        if let Some(moved_piece) = self.moved_piece.borrow().clone() {
            let piece_pointer_x = moved_piece.coords_x - (self.cells_size as f64) * 0.4;
            let piece_pointer_y = moved_piece.coords_y - (self.cells_size as f64) * 0.4;
            let image = self.pieces_images.get(&moved_piece.piece_type.char()).expect("Failed to get moved piece image !");

            cr.set_source_pixbuf(
                &image,
                piece_pointer_x,
                piece_pointer_y
            );
            cr.paint(); 
        }
    }

    fn draw_coordinates(&self, cr: &Context)
    {
        let files = ["A", "B", "C", "D", "E", "F", "G", "H"];
        let ranks = ["8", "7", "6", "5", "4", "3", "2", "1"];

        cr.set_source_rgb(0.2, 0.4, 1.0);
        cr.select_font_face(
            "Sans Serif",
            FontSlant::Normal,
            FontWeight::Bold
        );
        cr.set_font_size((self.cells_size as f64) * 0.38);
        
        (0..8).for_each(|file_index| {
            let real_file_index = if self.reversed { 7 - file_index } else { file_index };

            let letter = files[file_index];
            let letter_x = (self.cells_size as f64) * (0.9 + (real_file_index as f64));
            let letter_y_top = (self.cells_size as f64) * 0.4;
            let letter_y_bottom = (self.cells_size as f64) * 8.9;

            cr.move_to(letter_x, letter_y_top);
            cr.show_text(letter);

            cr.move_to(letter_x, letter_y_bottom);
            cr.show_text(letter);
        });

        (0..8).for_each(|rank_index| {
            let real_rank_index = if self.reversed { 7 - rank_index } else { rank_index };

            let letter = ranks[rank_index];
            let letter_y = (self.cells_size as f64) * (1.2 + (real_rank_index as f64));
            let letter_x_left = (self.cells_size as f64) * 0.1;
            let letter_x_right = (self.cells_size as f64) * 8.6;

            cr.move_to(letter_x_left, letter_y);
            cr.show_text(letter);

            cr.move_to(letter_x_right, letter_y);
            cr.show_text(letter);
        });
    }

    fn draw_player_turn(&self, cr: &Context)
    {
        let color = if self.logic.borrow().is_white_turn() { [1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0] };
        let center = (self.cells_size as f64) * 8.75;
        let radius = (self.cells_size as f64) * 0.25;
        cr.arc(center, center, radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgb(
            color[0],
            color[1],
            color[2],
        );
        cr.fill();
    }
}