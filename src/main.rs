use show_image::{ImageView, ImageInfo, create_window, WindowOptions, Color};
use show_image::{event};

const YIPPIE: &[u8] = include_bytes!("../y.gif");

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let image = image::open("y.gif").unwrap();

    // Create a window with default options and display the image.
    let window = create_window("yippie", WindowOptions {
        preserve_aspect_ratio: false,
        background_color: Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        },
        start_hidden: false,
        size: None,
        resizable: false,
        borderless: false,
        fullscreen: true,
        overlays_visible: false,
        default_controls: false,
    })?;
    window.set_image("image-001", image)?;

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }

    Ok(())
}