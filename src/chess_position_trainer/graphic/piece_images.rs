use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;

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
        let white_pawn = PieceImages::load_image(
            include_bytes!("../../resources/Chess_pl.png"),
            images_size
        ).expect("Failed to get white pawn image !");

        let white_knight = PieceImages::load_image(
            include_bytes!("../../resources/Chess_nl.png"),
            images_size
        ).expect("Failed to get white knight image !");

        let white_bishop = PieceImages::load_image(
            include_bytes!("../../resources/Chess_bl.png"),
            images_size
        ).expect("Failed to get white bishop image !");

        let white_rook = PieceImages::load_image(
            include_bytes!("../../resources/Chess_rl.png"),
            images_size
        ).expect("Failed to get white rook image !");

        let white_queen = PieceImages::load_image(
            include_bytes!("../../resources/Chess_ql.png"),
            images_size
        ).expect("Failed to get white queen image !");

        let white_king = PieceImages::load_image(
            include_bytes!("../../resources/Chess_kl.png"),
            images_size
        ).expect("Failed to get white king image !");

        let black_pawn = PieceImages::load_image(
            include_bytes!("../../resources/Chess_pd.png"),
            images_size
        ).expect("Failed to get black pawn image !");

        let black_knight = PieceImages::load_image(
            include_bytes!("../../resources/Chess_nd.png"),
            images_size
        ).expect("Failed to get black knight image !");

        let black_bishop = PieceImages::load_image(
            include_bytes!("../../resources/Chess_bd.png"),
            images_size
        ).expect("Failed to get black bishop image !");

        let black_rook = PieceImages::load_image(
            include_bytes!("../../resources/Chess_rd.png"),
            images_size
        ).expect("Failed to get black rook image !");

        let black_queen = PieceImages::load_image(
            include_bytes!("../../resources/Chess_qd.png"),
            images_size
        ).expect("Failed to get black queen image !");

        let black_king = PieceImages::load_image(
            include_bytes!("../../resources/Chess_kd.png"),
            images_size
        ).expect("Failed to get black king image !");

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

    fn load_image(image_bytes: &'static [u8], size: i32) -> Option<Pixbuf>
    {
        let image_stream = MemoryInputStream::new_from_bytes(
            &Bytes::from_static(image_bytes)
        );
        let image_pixbuf = Pixbuf::new_from_stream_at_scale(&image_stream, size, size, true, None);
        let image = match image_pixbuf {
            Ok(image) => Some(image),
            Err(e) => {
                println!("Failed to get image ! ({})", e);
                None
            }
        };
        image
    }
}