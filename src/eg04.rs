extern crate sdl2;

use std::path::Path;

use sdl2::event::{Event};
use sdl2::surface::{Surface,SurfaceRef};

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx.window("eg04", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Load a surface.
    // Surfaces live in system RAM, so they aren't ideal for performance.
    let surface = match Surface::load_bmp(&Path::new("res/ice-troll.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load surface: {}", err)
    };

    let mut events = ctx.event_pump().unwrap();

    {
        {
            // get the surface used by our window
            let screen = window.surface(&events).unwrap();

            // copy our surface unto the window's surface
            unsafe {
                // this is somewhat ugly, but in the current state
                // there is no easy SurfaceRef -> Surface conversion
                let _ = surface.blit(None, SurfaceRef::from_ll_mut(screen.raw()), None);
            }
        }

        {
            // update the window to display the changed surface
            match window.update_surface() {
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

