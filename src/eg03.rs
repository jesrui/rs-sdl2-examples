extern crate sdl2;

use sdl2::event::{Event};
use sdl2::rect::{Rect};

fn main() {
    // start sdl2 with everything
    let mut ctx = sdl2::init().everything().unwrap();

    // Create a window
    let window  = match ctx.window("eg03", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Create a rendering context
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    // since the drawer changes, it should be borrowed as mutable
    let mut drawer = renderer.drawer();

    // Set the drawing color to a light blue.
    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(101, 208, 246));

    // Clear the buffer, using the light blue color set above.
    let _ = drawer.clear();

    // Set the drawing color to a darker blue.
    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 153, 204));

    // Create centered Rect, draw the outline of the Rect in our dark blue color.
    let border_rect = Rect::new(320-64, 240-64, 128, 128);
    let _ = drawer.draw_rect(border_rect);

    // Create a smaller centered Rect, filling it in the same dark blue.
    let inner_rect = Rect::new(320-60, 240-60, 120, 120);
    let _ = drawer.fill_rect(inner_rect);

    // Swap our buffer for the present buffer, displaying it.
    let _ = drawer.present();

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

