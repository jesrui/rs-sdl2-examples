extern crate sdl2;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::event::{Event, poll_event};
use sdl2::rect::{Rect};

fn main() {
    // start sdl2 with everything
    sdl2::init(sdl2::INIT_EVERYTHING);

    // Create a window
    let window  = match Window::new("eg03", WindowPos::PosCentered, WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Create a rendering context
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    // Set the drawing color to a light blue.
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(101, 208, 246));

    // Clear the buffer, using the light blue color set above.
    let _ = renderer.clear();

    // Set the drawing color to a darker blue.
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 153, 204));

    // Create centered Rect, draw the outline of the Rect in our dark blue color.
    let border_rect = Rect::new(320-64, 240-64, 128, 128);
    let _ = match renderer.draw_rect(&border_rect) {
        Ok(_)    => {},
        Err(err) => panic!("failed to draw rect: {}", err) 
    };

    // Create a smaller centered Rect, filling it in the same dark blue.
    let inner_rect = Rect::new(320-60, 240-60, 120, 120);
    let _ = match renderer.fill_rect(&inner_rect) {
        Ok(_)    => {},
        Err(err) => panic!("failed to draw rect: {}", err) 
    };

    // Swap our buffer for the present buffer, displaying it.
    let _ = renderer.present();

    // loop until we receive a QuitEvent
    'event : loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            _            => continue
        }
    }

    sdl2::quit();
}

