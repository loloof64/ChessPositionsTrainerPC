use gdk_pixbuf::Pixbuf;
use chess_position_trainer::graphic::load_image;

pub struct PieceImages
{
    
}

impl PieceImages
{

    pub fn get_white_pawn(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_pl.png"),
            size
        ).expect("Failed to get white pawn image !")
    }

    pub fn get_white_knight(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_nl.png"),
            size
        ).expect("Failed to get white knight image !")
    }

    pub fn get_white_bishop(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_bl.png"),
            size
        ).expect("Failed to get white bishop image !")
    }

    pub fn get_white_rook(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_rl.png"),
            size
        ).expect("Failed to get white rook image !")
    }

    pub fn get_white_queen(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_ql.png"),
            size
        ).expect("Failed to get white queen image !")
    }

    pub fn get_white_king(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_kl.png"),
            size
        ).expect("Failed to get white king image !")
    }

    pub fn get_black_pawn(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_pd.png"),
            size
        ).expect("Failed to get black pawn image !")
    }

    pub fn get_black_knight(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_nd.png"),
            size
        ).expect("Failed to get black knight image !")
    }

    pub fn get_black_bishop(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_bd.png"),
            size
        ).expect("Failed to get black bishop image !")
    }

    pub fn get_black_rook(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_rd.png"),
            size
        ).expect("Failed to get black rook image !")
    }

    pub fn get_black_queen(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_qd.png"),
            size
        ).expect("Failed to get black queen image !")
    }

    pub fn get_black_king(size: i32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_kd.png"),
            size
        ).expect("Failed to get black king image !")
    }
}