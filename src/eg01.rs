extern crate sdl2;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::timer::{delay};

fn main() {
    // start sdl2

    let ctx = match sdl2::init(sdl2::INIT_VIDEO) {
        Ok(ctx)  => ctx,
        Err(err) => panic!("Failed to start SDL2: {}", err)
    };

    // Create a window

    let mut window = match Window::new(&ctx, "eg01", WindowPos::PosCentered, WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let event_pump = ctx.event_pump();
    let mut window_properties = window.properties(&event_pump);

    // Display the window for 3 seconds

    window_properties.show();
    delay(3000);
}

