use gtk::prelude::*;
use gdk::prelude::*;
use gtk::DrawingArea;
use cairo::Context;
use cairo::enums::{FontSlant, FontWeight};
use shakmaty::Role;
use chess_position_trainer::graphic::PieceImages;
use chess_position_trainer::logic::chessgame::ChessGame;

pub struct ChessBoard
{
    drawing_area: DrawingArea,
}

impl ChessBoard
{
    pub fn new_from_default(cells_size: u32) -> Option<ChessBoard>
    {
        ChessBoard::get_chessboard(
            cells_size,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        )
    }

    pub fn new_from_fen(cells_size: u32, initial_position: &str) -> Option<ChessBoard>
    {
        ChessBoard::get_chessboard(
            cells_size,
            initial_position,
        )
    }

    pub fn get_drawing_area(&self) -> &DrawingArea
    {
        &self.drawing_area
    }

    fn get_chessboard(cells_size: u32, initial_position: &str) -> Option<ChessBoard>
    {
        let piece_images = PieceImages::new(cells_size as i32);
        let drawing_area = DrawingArea::new();

        let logic = ChessGame::new_from_fen(initial_position);

        match logic {
            Some(game_logic) => {
                drawing_area.connect_draw(move |_, cr|{
                    ChessBoard::draw_background(cr);
                    ChessBoard::draw_cells(cr, cells_size);
                    ChessBoard::draw_pieces(cr, cells_size, &game_logic, &piece_images);
                    ChessBoard::draw_coordinates(cr, cells_size);
                    ChessBoard::draw_player_turn(cr, cells_size, &game_logic);

                    Inhibit(false)
                });

                Some(ChessBoard {
                    drawing_area,
                })
            },
            _ => None
        }
    }

    fn draw_background(cr: &Context)
    {
        let green_color = [60.0/255.0, 204.0/255.0, 100.0/255.0];
        cr.set_source_rgb(
            green_color[0],
            green_color[1],
            green_color[2],
        );
        cr.paint();
    }

    fn draw_cells(cr: &Context, cells_size: u32)
    {
        (0..8).for_each(|rank| {
            (0..8).for_each(|file| {
                let white_cell_color = [255.0/255.0, 255.0/255.0, 179.0/255.0];
                let black_cell_color = [153.0/255.0, 102.0/255.0, 51.0/255.0];

                let is_white_cell = (file + rank) % 2 == 0;
                let cell_color = if is_white_cell {white_cell_color} else {black_cell_color};

                let rect_x = (cells_size as f64) * (0.5 + (file as f64));
                let rect_y = (cells_size as f64) * (0.5 + (rank as f64));
                let rect_size = cells_size as f64;

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

    fn draw_pieces(cr: &Context, cells_size: u32, 
        logic: &ChessGame, piece_images: &PieceImages)
    {
        (0..8).for_each(|rank| {
            (0..8).for_each(|file| {
                if let Some(piece) = logic.piece_at_cell(file, rank) {
                    let image = match piece.role {
                        Role::Pawn => {
                            if piece.color.is_white() 
                            {
                                piece_images.get_white_pawn()
                            }
                            else
                            {
                                piece_images.get_black_pawn()
                            }
                        },
                        Role::Knight => {
                            if piece.color.is_white() 
                            {
                                piece_images.get_white_knight()
                            }
                            else
                            {
                                piece_images.get_black_knight()
                            }
                        },
                        Role::Bishop => {
                            if piece.color.is_white() 
                            {
                                piece_images.get_white_bishop()
                            }
                            else
                            {
                                piece_images.get_black_bishop()
                            }
                        },
                        Role::Rook => {
                            if piece.color.is_white() 
                            {
                                piece_images.get_white_rook()
                            }
                            else
                            {
                                piece_images.get_black_rook()
                            }
                        },
                        Role::Queen => {
                            if piece.color.is_white() 
                            {
                                piece_images.get_white_queen()
                            }
                            else
                            {
                                piece_images.get_black_queen()
                            }
                        },
                        Role::King => {
                            if piece.color.is_white() 
                            {
                                piece_images.get_white_king()
                            }
                            else
                            {
                                piece_images.get_black_king()
                            }
                        },
                    };

                    let location_x = (cells_size as f64) * (file as f64 + 0.5);
                    let location_y = (cells_size as f64) * ((7.0-rank as f64) + 0.5);
                    cr.set_source_pixbuf(
                        image,
                        location_x,
                        location_y
                    );
                    cr.paint();   
                }
            });
        });
    }

    fn draw_coordinates(cr: &Context, cells_size: u32)
    {
        let files = ["A", "B", "C", "D", "E", "F", "G", "H"];
        let ranks = ["8", "7", "6", "5", "4", "3", "2", "1"];

        cr.set_source_rgb(0.2, 0.4, 1.0);
        cr.select_font_face(
            "Sans Serif",
            FontSlant::Normal,
            FontWeight::Bold
        );
        cr.set_font_size((cells_size as f64) * 0.38);
        
        (0..8).for_each(|file_index| {
            let letter = files[file_index];
            let letter_x = (cells_size as f64) * (0.9 + (file_index as f64));
            let letter_y_top = (cells_size as f64) * 0.4;
            let letter_y_bottom = (cells_size as f64) * 8.9;

            cr.move_to(letter_x, letter_y_top);
            cr.show_text(letter);

            cr.move_to(letter_x, letter_y_bottom);
            cr.show_text(letter);
        });

        (0..8).for_each(|rank_index| {
            let letter = ranks[rank_index];
            let letter_y = (cells_size as f64) * (1.2 + (rank_index as f64));
            let letter_x_left = (cells_size as f64) * 0.1;
            let letter_x_right = (cells_size as f64) * 8.6;

            cr.move_to(letter_x_left, letter_y);
            cr.show_text(letter);

            cr.move_to(letter_x_right, letter_y);
            cr.show_text(letter);
        });
    }

    fn draw_player_turn(cr: &Context, cells_size: u32, logic: &ChessGame)
    {
        let color = if logic.is_white_turn() { [1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0] };
        let center = (cells_size as f64) * 8.75;
        let radius = (cells_size as f64) * 0.25;
        cr.arc(center, center, radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgb(
            color[0],
            color[1],
            color[2],
        );
        cr.fill();
    }
}