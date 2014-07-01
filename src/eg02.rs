extern crate sdl2;
extern crate native;

use sdl2::video::{Window, PosCentered, OpenGL};
use sdl2::event::{QuitEvent, NoEvent, poll_event};

fn main() {
    // start sdl2 with everything
    sdl2::init(sdl2::InitEverything);

    // Create a window and show it
    let window  = match Window::new("eg02", PosCentered, PosCentered, 640, 480, OpenGL) {
        Ok(window) => window,
        Err(err)   => fail!("failed to create window: {}", err)
    };
    window.show();

    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened
        match poll_event() {
            QuitEvent(_) => break 'event,
            NoEvent      => continue,
            // print the event to the console
            event        => println!("event: {}", event),
        }
    }

    sdl2::quit();
}

