extern crate sdl2;

use std::path::Path;

use sdl2::event::{Event};
use sdl2::surface::{Surface};

fn main() {
    let mut ctx = sdl2::init().everything().unwrap();

    let mut window  = match ctx.window("eg04", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Load a surface.
    // Surfaces live in system RAM, so they aren't ideal for performance.
    let surface = match Surface::load_bmp(&Path::new("res/ice-troll.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load surface: {}", err)
    };

    {
        let mut window_properties = window.properties(&ctx);
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

    let mut events = ctx.event_pump();

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

