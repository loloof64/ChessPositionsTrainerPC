use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;

pub struct MiscImages
{
    up_down: Pixbuf,
}

impl MiscImages {

    pub fn new() -> MiscImages {
        let up_down = PieceImages::load_image(
            include_bytes!("../../resources/UpDown.png"),
            60,
        ).expect("Failed to get upDown image !");

        MiscImages {
            up_down,
        }
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