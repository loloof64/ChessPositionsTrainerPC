use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gdk::prelude::*;
use gdk::{EventMask, EventType};
use gtk::DrawingArea;
use cairo::Context;
use cairo::enums::{FontSlant, FontWeight};
use shakmaty::Role;
use chess_position_trainer::graphic::PieceImages;
use chess_position_trainer::logic::chessgame::ChessGame;

#[derive(Clone)]
pub struct ChessBoard
{
    drawing_area: DrawingArea,
    reversed: bool,
    logic: ChessGame,
    cells_size: u32,
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
                let chess_board = ChessBoard {
                    drawing_area,
                    reversed: false,
                    logic: game_logic,
                    cells_size: 50u32,
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
                        EventType::ButtonPress => chess_board_ref_3.borrow().handle_mouse_pressed(coords),
                        EventType::ButtonRelease => chess_board_ref_3.borrow().handle_mouse_released(coords),
                        EventType::MotionNotify => chess_board_ref_3.borrow().handle_mouse_moved(coords),
                    _ => {} 
                    }
                    Inhibit(false)
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

        println!("Button pressed at {:?} !", cell_coords);
    }

    fn handle_mouse_released(&self, coords: (f64, f64)){
        let cells_size = self.cells_size as f64;
        let mut cell_coords = (
            ((coords.0 - (cells_size * 0.5)) / cells_size) as i32,
            7 - (((coords.1 - (cells_size * 0.5)) / cells_size) as i32),
        );
        if self.reversed {
            cell_coords = (7-cell_coords.0, 7-cell_coords.1);
        }

        println!("Button released at {:?} !", cell_coords);
    }

    fn handle_mouse_moved(&self, coords: (f64, f64)){
        let cells_size = self.cells_size as f64;
        let mut cell_coords = (
            ((coords.0 - (cells_size * 0.5)) / cells_size) as i32,
            7 - (((coords.1 - (cells_size * 0.5)) / cells_size) as i32),
        );
        if self.reversed {
            cell_coords = (7-cell_coords.0, 7-cell_coords.1);
        }

        println!("Button moved at {:?} !", cell_coords);
    }

    fn paint(&self, cr: &Context){
        self.draw_background(cr);
        self.draw_cells(cr);
        self.draw_pieces(cr);
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
        (0..8).for_each(|rank| {
            (0..8).for_each(|file| {
                let real_file = if self.reversed { 7-file } else { file };
                let real_rank = if self.reversed { 7-rank } else { rank };

                let piece_size = (self.cells_size as f64 * 0.8) as i32;

                if let Some(piece) = self.logic.piece_at_cell(real_file, real_rank) {
                    let image = match piece.role {
                        Role::Pawn => {
                            if piece.color.is_white() 
                            {
                                PieceImages::get_white_pawn(piece_size)
                            }
                            else
                            {
                                PieceImages::get_black_pawn(piece_size)
                            }
                        },
                        Role::Knight => {
                            if piece.color.is_white() 
                            {
                                PieceImages::get_white_knight(piece_size)
                            }
                            else
                            {
                                PieceImages::get_black_knight(piece_size)
                            }
                        },
                        Role::Bishop => {
                            if piece.color.is_white() 
                            {
                                PieceImages::get_white_bishop(piece_size)
                            }
                            else
                            {
                                PieceImages::get_black_bishop(piece_size)
                            }
                        },
                        Role::Rook => {
                            if piece.color.is_white() 
                            {
                                PieceImages::get_white_rook(piece_size)
                            }
                            else
                            {
                                PieceImages::get_black_rook(piece_size)
                            }
                        },
                        Role::Queen => {
                            if piece.color.is_white() 
                            {
                                PieceImages::get_white_queen(piece_size)
                            }
                            else
                            {
                                PieceImages::get_black_queen(piece_size)
                            }
                        },
                        Role::King => {
                            if piece.color.is_white() 
                            {
                                PieceImages::get_white_king(piece_size)
                            }
                            else
                            {
                                PieceImages::get_black_king(piece_size)
                            }
                        },
                    };

                    let location_x = (self.cells_size as f64) * (file as f64 + 0.5 + 0.1);
                    let location_y = (self.cells_size as f64) * ((7.0-rank as f64) + 0.5 + 0.1);
                    cr.set_source_pixbuf(
                        &image,
                        location_x,
                        location_y
                    );
                    cr.paint();   
                }
            });
        });
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