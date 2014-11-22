extern crate sdl2;

use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::event::{Quit, None, poll_event};

fn main() {
    // start sdl2 with everything
    sdl2::init(sdl2::INIT_EVERYTHING);

    // Create a window and show it
    let window  = match Window::new("eg02", PosCentered, PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };
    window.show();

    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened
        match poll_event() {
            Quit(_) => break 'event,
            None => continue,
            // print the event to the console
            event        => println!("event: {}", event),
        }
    }

    sdl2::quit();
}

