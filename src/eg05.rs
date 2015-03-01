extern crate sdl2;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::event::{Event};
use sdl2::surface::{Surface};

fn main() {
    let ctx = match sdl2::init(sdl2::INIT_EVERYTHING) {
        Ok(ctx)  => ctx,
        Err(err) => panic!("Failed to start SDL2: {}", err)
    };

    let window  = match Window::new("eg05", WindowPos::PosCentered, WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    // Load a surface.
    // Surfaces live in system RAM, so they aren't ideal for performance.
    let surface = match Surface::from_bmp(&Path::new("res/ice-troll.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load surface: {}", err)
    };

    // Convert a surface to a texture.
    // Textures can be used more efficiently by the GPU. (If one is available.)
    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(err)    => panic!("failed to convert surface: {}", err)
    };

    let mut drawer = renderer.drawer();

    let _ = drawer.clear();
    // Display the texture.
    // Omitting the src & dst Rect arguments will cause our image to stretch across the entire buffer.
    // Try passing Some(surface.get_rect()) for src & dst instead of None & see how things change.
    let _ = drawer.copy(&texture, None, None);
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

