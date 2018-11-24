use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;
use chess_position_trainer::graphic::load_image;

pub struct MiscImages
{
    up_down: Pixbuf,
}

impl MiscImages {

    pub fn new() -> MiscImages {
        let up_down = load_image(
            include_bytes!("../../resources/UpDown.png"),
            60,
        ).expect("Failed to get upDown image !");

        MiscImages {
            up_down,
        }
    }

}