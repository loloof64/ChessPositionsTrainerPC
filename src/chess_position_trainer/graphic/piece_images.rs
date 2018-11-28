use gdk_pixbuf::Pixbuf;
use chess_position_trainer::graphic::load_image;

pub struct PieceImages
{
    
}

impl PieceImages
{

    pub fn get_white_pawn(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_pl.png"),
            size as i32
        ).expect("Failed to get white pawn image !")
    }

    pub fn get_white_knight(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_nl.png"),
            size as i32
        ).expect("Failed to get white knight image !")
    }

    pub fn get_white_bishop(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_bl.png"),
            size as i32
        ).expect("Failed to get white bishop image !")
    }

    pub fn get_white_rook(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_rl.png"),
            size as i32
        ).expect("Failed to get white rook image !")
    }

    pub fn get_white_queen(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_ql.png"),
            size as i32
        ).expect("Failed to get white queen image !")
    }

    pub fn get_white_king(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_kl.png"),
            size as i32
        ).expect("Failed to get white king image !")
    }

    pub fn get_black_pawn(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_pd.png"),
            size as i32
        ).expect("Failed to get black pawn image !")
    }

    pub fn get_black_knight(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_nd.png"),
            size as i32
        ).expect("Failed to get black knight image !")
    }

    pub fn get_black_bishop(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_bd.png"),
            size as i32
        ).expect("Failed to get black bishop image !")
    }

    pub fn get_black_rook(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_rd.png"),
            size as i32
        ).expect("Failed to get black rook image !")
    }

    pub fn get_black_queen(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_qd.png"),
            size as i32
        ).expect("Failed to get black queen image !")
    }

    pub fn get_black_king(size: u32) -> Pixbuf
    {
        load_image(
            include_bytes!("../../resources/Chess_kd.png"),
            size as i32
        ).expect("Failed to get black king image !")
    }
}