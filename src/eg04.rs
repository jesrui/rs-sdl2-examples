extern crate sdl2;

use std::path::Path;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::event::{Event};
use sdl2::surface::{Surface};

fn main() {
    let ctx = match sdl2::init(sdl2::INIT_EVERYTHING) {
        Ok(ctx)  => ctx,
        Err(err) => panic!("Failed to start SDL2: {}", err)
    };

    let mut window = match Window::new(&ctx, "eg04", WindowPos::PosCentered, WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Load a surface.
    // Surfaces live in system RAM, so they aren't ideal for performance.
    let surface = match Surface::from_bmp(&Path::new("res/ice-troll.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load surface: {}", err)
    };

    let mut events = ctx.event_pump();
    {
        let mut window_properties = window.properties(&events);
        {
            // get the surface used by our window
            let screen = window_properties.get_surface().unwrap();

            // copy our surface unto the window's surface
            screen.blit(&surface, None, None);
        }

        {
            // update the window to display the changed surface
            match window_properties.update_surface() {
                Ok(_) => {},
                Err(err) => panic!("failed to update window surface: {}", err)
            }
        }
    }

    // loop until we receive a QuitEvent
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => continue
            }
        }
    }
}

