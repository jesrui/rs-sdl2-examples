extern crate sdl2;

use sdl2::timer::{delay};

fn main() {
    // start sdl2

    let ctx = sdl2::init().video().unwrap();

    // Create a window

    let mut window = match ctx.window("eg01", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut window_properties = window.properties(&ctx);

    // Display the window for 3 seconds

    window_properties.show();
    delay(3000);
}

