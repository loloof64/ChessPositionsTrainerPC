use gtk::prelude::*;
use gtk::{DrawingArea};

pub struct ChessBoard
{
    drawing_area: DrawingArea,
    cells_size: u32,
}

impl ChessBoard
{
    pub fn new(cells_size: u32) -> ChessBoard
    {
        let drawing_area = DrawingArea::new();
        drawing_area.connect_draw(|_, cr|{
            let white_cell_color = [255.0/255.0, 255.0/255.0, 179.0/255.0];
            cr.set_source_rgb(
                white_cell_color[0],
                white_cell_color[1],
                white_cell_color[2],
            );
            cr.paint();

            Inhibit(false)
        });
        
        ChessBoard {
            drawing_area: drawing_area,
            cells_size: cells_size,
        }
    }

    pub fn get_drawing_area(&self) ->  &DrawingArea 
    {
        &self.drawing_area
    }
}