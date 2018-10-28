use gtk::prelude::*;
use gdk::prelude::*;
use gtk::DrawingArea;
use cairo::Context;
use cairo::enums::FontSlant;
use cairo::enums::FontWeight;
use chess_position_trainer::graphic::PieceImages;

pub struct ChessBoard
{
    drawing_area: DrawingArea,
    cells_size: u32,
}

impl ChessBoard
{
    pub fn new(cells_size: u32) -> ChessBoard
    {
        let piece_images = PieceImages::new(cells_size as i32);
        let drawing_area = DrawingArea::new();

        drawing_area.connect_draw(move |_, cr|{
            fn draw_background(cr: &Context)
            {
                let pink_color = [255.0/255.0, 204.0/255.0, 204.0/255.0];
                cr.set_source_rgb(
                    pink_color[0],
                    pink_color[1],
                    pink_color[2],
                );
                cr.paint();
            }

            fn draw_cells(cr: &Context, cells_size: u32)
            {
                (0..8).for_each(|row| {
                    (0..8).for_each(|col| {
                        let white_cell_color = [255.0/255.0, 255.0/255.0, 179.0/255.0];
                        let black_cell_color = [153.0/255.0, 102.0/255.0, 51.0/255.0];

                        let is_white_cell = (row+col) % 2 == 0;
                        let cell_color = if is_white_cell {white_cell_color} else {black_cell_color};

                        let rect_x = (cells_size as f64) * (0.5 + (col as f64));
                        let rect_y = (cells_size as f64) * (0.5 + (row as f64));
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

            fn draw_pieces(cr: &Context, cells_size: u32, piece_images: &PieceImages)
            {
                let image = piece_images.get_white_queen();
                let location_x = (cells_size as f64) * 0.5;
                let location_y = (cells_size as f64) * 0.5;
                cr.set_source_pixbuf(
                    image,
                    location_x,
                    location_y
                );
                cr.paint();
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

            draw_background(cr);
            draw_cells(cr, cells_size);
            draw_pieces(cr, cells_size, &piece_images);
            draw_coordinates(cr, cells_size);

            Inhibit(false)
        });
        
        ChessBoard {
            drawing_area: drawing_area,
            cells_size: cells_size,
        }
    }

    pub fn get_drawing_area(&self) -> &DrawingArea
    {
        &self.drawing_area
    }
}