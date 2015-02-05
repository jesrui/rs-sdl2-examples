#![feature(path)]

extern crate sdl2;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::event::{Event, poll_event};
use sdl2::surface::{Surface};

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window  = match Window::new("eg04", WindowPos::PosCentered, WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Load a surface.
    // Surfaces live in system RAM, so they aren't ideal for performance.
    let surface = match Surface::from_bmp(&Path::new("res/ice-troll.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load surface: {}", err)
    };

    // get the surface used by our window
    let screen = match window.get_surface() {
        Ok(s) => s,
        Err(err) => panic!("failed to get window surface: {}", err)
    };

    // copy our surface unto the window's surface
    screen.blit(&surface, None, None);

    // update the window to display the changed surface
    window.update_surface();

    // loop until we receive a QuitEvent
    'event : loop {
        match poll_event() {
            Event::Quit{..} => break 'event,
            _               => continue
        }
    }

    sdl2::quit();
}

