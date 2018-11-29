use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gdk::prelude::*;
use gdk::{EventMask, EventType};
use gtk::DrawingArea;
use gdk_pixbuf::Pixbuf;
use cairo::Context;
use cairo::enums::{FontSlant, FontWeight};
use shakmaty::{Piece, Chess};
use chess_position_trainer::graphic::load_image;
use chess_position_trainer::logic::chessgame::ChessGame;

#[derive(Clone)]
pub struct ChessBoard
{
    drawing_area: DrawingArea,
    reversed: bool,
    logic: ChessGame,
    cells_size: u32,
    moved_piece: Option<MovedPiece>,
    images: HashMap<char, Pixbuf>,
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
                let images = ChessBoard::load_pieces_images((50f64 * 0.8) as u32);

                let chess_board = ChessBoard {
                    drawing_area,
                    reversed: false,
                    logic: game_logic,
                    cells_size: 50u32,
                    moved_piece: None,
                    images,
                };

                let chess_board_ref = Rc::new(RefCell::new(chess_board));

                let chess_board_ref_2 = chess_board_ref.clone();
                chess_board_ref.borrow().drawing_area.connect_draw(move |_drawing_area, cr|{
                    chess_board_ref_2.borrow().paint(cr);
                    Inhibit(false)
                });

                let chess_board_ref_3 = chess_board_ref.clone();
                chess_board_ref.borrow().drawing_area.connect_event(move |_self, event| {
                    let coords = event.get_coords().expect("Failed to get mouse coordinates !");
                    
                    match event.get_event_type() {
                        EventType::ButtonPress => chess_board_ref_3.borrow_mut().handle_mouse_pressed(coords),
                        EventType::ButtonRelease => chess_board_ref_3.borrow_mut().handle_mouse_released(coords),
                        EventType::MotionNotify => chess_board_ref_3.borrow_mut().handle_mouse_moved(coords),
                    _ => {} 
                    }
                    Inhibit(false)
                });

                Ok(chess_board_ref)
            },
            None => Err(format!("Bad FEN {} !", initial_position))
        }
    }

    fn handle_mouse_pressed(&mut self, coords: (f64, f64)){
        let cells_size = self.cells_size as f64;
        let mut cell_coords = (
            ((coords.0 - (cells_size * 0.5)) / cells_size) as i32,
            7 - (((coords.1 - (cells_size * 0.5)) / cells_size) as i32),
        );
        if self.reversed {
            cell_coords = (7-cell_coords.0, 7-cell_coords.1);
        }

        let (coords_x, coords_y) = coords;
        let moved_piece = self.logic.piece_at_cell(cell_coords.0 as i8, cell_coords.1 as i8);
        if let Some(piece_type) = moved_piece {
            self.moved_piece = Some(MovedPiece{
                coords_x,
                coords_y,
                piece_type,
                start_file: cell_coords.0 as u8,
                start_rank: cell_coords.1 as u8,
            });

            self.drawing_area.queue_draw();
        }
    }

    fn handle_mouse_released(&mut self, coords: (f64, f64)){
        let cells_size = self.cells_size as f64;
        let mut cell_coords = (
            ((coords.0 - (cells_size * 0.5)) / cells_size) as u8,
            7 - (((coords.1 - (cells_size * 0.5)) / cells_size) as u8),
        );
        if self.reversed {
            cell_coords = ((7-cell_coords.0) as u8, (7-cell_coords.1) as u8);
        }

        if let Some(ref moved_piece) = self.moved_piece {
            let start_cell = (moved_piece.start_file, moved_piece.start_rank);
            let end_cell = cell_coords;

            if self.logic.is_legal_move::<Chess>(start_cell, end_cell, None) {
                self.logic.do_move::<Chess>(start_cell, end_cell, None);
                self.drawing_area.queue_draw();
            }
        }

        self.moved_piece = None;
        self.drawing_area.queue_draw();
    }

    fn handle_mouse_moved(&mut self, coords: (f64, f64)){
        if let Some(ref mut move_spec) = self.moved_piece {
            move_spec.translate_to(coords.0, coords.1);
            self.drawing_area.queue_draw();

        }
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

            if let Some(piece) = self.logic.piece_at_cell(real_file as i8, real_rank as i8) {
                let not_moved_piece = match self.moved_piece {
                    None => true,
                    Some(ref moved_piece) => moved_piece.start_file != real_file || moved_piece.start_rank != real_rank
                };

                if not_moved_piece {
                        let image = self.images.get(&piece.char()).expect("Failed to get piece image !");
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
        if let Some(ref moved_piece) = self.moved_piece {
            let piece_pointer_x = moved_piece.coords_x - (self.cells_size as f64) * 0.4;
            let piece_pointer_y = moved_piece.coords_y - (self.cells_size as f64) * 0.4;
            let image = self.images.get(&moved_piece.piece_type.char()).expect("Failed to get moved piece image !");

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
        let color = if self.logic.is_white_turn() { [1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0] };
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