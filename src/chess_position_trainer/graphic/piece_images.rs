use gdk_pixbuf::Pixbuf;
use std::path::Path;

pub struct PieceImages
{
    white_pawn: Pixbuf,
    white_knight: Pixbuf,
    white_bishop: Pixbuf,
    white_rook: Pixbuf,
    white_queen: Pixbuf,
    white_king: Pixbuf,

    black_pawn: Pixbuf,
    black_knight: Pixbuf,
    black_bishop: Pixbuf,
    black_rook: Pixbuf,
    black_queen: Pixbuf,
    black_king: Pixbuf,
}

impl PieceImages
{
    pub fn new(images_size: i32) -> PieceImages
    {
        let white_pawn = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_pl.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let white_knight = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_nl.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let white_bishop = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_bl.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let white_rook = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_rl.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let white_queen = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_ql.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let white_king = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_kl.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let black_pawn = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_pd.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let black_knight = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_nd.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let black_bishop = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_bd.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let black_rook = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_rd.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let black_queen = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_qd.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        let black_king = Pixbuf::new_from_file_at_scale(
            Path::new("resources/Chess_kd.png"),
            images_size,
            images_size,
            true
        ).ok().unwrap();

        PieceImages 
        {
            white_pawn,
            white_knight,
            white_bishop,
            white_rook,
            white_queen,
            white_king,

            black_pawn,
            black_knight,
            black_bishop,
            black_rook,
            black_queen,
            black_king,
        }
    }

    pub fn get_white_pawn(&self) -> &Pixbuf
    {
        &self.white_pawn
    }

    pub fn get_white_knight(&self) -> &Pixbuf
    {
        &self.white_knight
    }

    pub fn get_white_bishop(&self) -> &Pixbuf
    {
        &self.white_bishop
    }

    pub fn get_white_rook(&self) -> &Pixbuf
    {
        &self.white_rook
    }

    pub fn get_white_queen(&self) -> &Pixbuf
    {
        &self.white_queen
    }

    pub fn get_white_king(&self) -> &Pixbuf
    {
        &self.white_king
    }

    pub fn get_black_pawn(&self) -> &Pixbuf
    {
        &self.black_pawn
    }

    pub fn get_black_knight(&self) -> &Pixbuf
    {
        &self.black_knight
    }

    pub fn get_black_bishop(&self) -> &Pixbuf
    {
        &self.black_bishop
    }

    pub fn get_black_rook(&self) -> &Pixbuf
    {
        &self.black_rook
    }

    pub fn get_black_queen(&self) -> &Pixbuf
    {
        &self.black_queen
    }

    pub fn get_black_king(&self) -> &Pixbuf
    {
        &self.black_king
    }
}