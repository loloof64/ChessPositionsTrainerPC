use gdk_pixbuf::Pixbuf;
use std::path::Path;

pub struct PieceImages
{
    white_queen: Pixbuf
}

impl PieceImages
{
    pub fn new(images_size: i32) -> PieceImages
    {
        let white_queen = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_ql.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        PieceImages 
        {
            white_queen: white_queen,
        }
    }

    pub fn get_white_queen(&self) -> &Pixbuf
    {
        &self.white_queen
    }
}