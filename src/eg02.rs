extern crate sdl2;

use sdl2::event::{Event};

fn main() {
    // start sdl2 with everything
    let mut ctx = sdl2::init().everything().unwrap();

    // Create a window and show it
    let mut window  = match ctx.window("eg02", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    {
        let mut window_properties = window.properties(&ctx);
        window_properties.show();
    }

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

