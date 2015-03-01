extern crate sdl2;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::event::{Event};

fn main() {
    // start sdl2 with everything
    let ctx = match sdl2::init(sdl2::INIT_EVERYTHING) {
        Ok(ctx)  => ctx,
        Err(err) => panic!("Failed to start SDL2: {}", err)
    };

    // Create a window and show it
    let window  = match Window::new("eg02", WindowPos::PosCentered, WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };
    window.show();

    let mut events = ctx.event_pump();

    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => continue
            }
        }
    }
}

