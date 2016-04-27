/*
This is just a translation to Rust of lesson 3 of Will Usher's SDL
tutorial. See
http://www.willusher.io/sdl2%20tutorials/2013/08/18/lesson-3-sdl-extension-libraries

In this example we'll see how to load images in PNG format, and scale them to fill
a tiled background.
 */

extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::event::{Event,WindowEventId};
use sdl2::render::{Renderer,Texture,TextureQuery};
use sdl2::rect::{Rect};
use sdl2::keyboard::Keycode;

use sdl2_image::{LoadTexture, INIT_PNG};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
//We'll just be using square tiles for now
const TILE_SIZE: u32 = 40;

/*
 * Draw an SDL_Texture to an SDL Renderer at position x, y, preserving
 * the texture's width and height
 * @param texture The source texture we want to draw
 * @param renderer The renderer we want to draw to
 * @param x The x coordinate to draw to
 * @param y The y coordinate to draw to
 */
fn render_texture(texture: &Texture, renderer: &mut Renderer, x: i32, y: i32) {
    let TextureQuery { width, height, .. } = texture.query();
    let dst = Rect::new(x, y, width, height);
    renderer.copy(&texture, None, Some(dst));
}

fn main() {
    let ctx = match sdl2::init() {
        Ok(ctx) => ctx,
        Err(err) => panic!("failed to init sdl2 lib: {}", err)
    };
    // you can try also init(INIT_PNG | INIT_JPG)
    let _image_ctx = match sdl2_image::init(INIT_PNG) {
        Ok(ctx) => ctx,
        Err(err) => panic!("failed to init sdl2 image: {}", err)
    };
    let video_ctx = match ctx.video() {
        Ok(ctx) => ctx,
        Err(err) => panic!("failed to init sdl2 video: {}", err)
    };

    let window  = match video_ctx.window("eg07", SCREEN_WIDTH, SCREEN_HEIGHT)
                .position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    renderer.clear();

    // Load a tile image of the background into a texture.
    let background = match renderer.load_texture(&Path::new("res/tile-eg07.png")) {
        Ok(texture) => texture,
        Err(err)    => panic!("failed to convert surface: {}", err)
    };

    // Display the tiled background.
    // Determine how many tiles we'll need to fill the screen.
    let x_tiles = SCREEN_WIDTH / TILE_SIZE;
    let y_tiles = SCREEN_HEIGHT / TILE_SIZE;
    //Draw the tiles by calculating their positions
    for i in 0 .. x_tiles*y_tiles {
        let x = i % x_tiles;
        let y = i / x_tiles;
        let dst = Rect::new((x*TILE_SIZE) as i32, (y*TILE_SIZE) as i32,
                            TILE_SIZE, TILE_SIZE);
        renderer.copy(&background,None,Some(dst));
    }

    let image = match renderer.load_texture(&Path::new("res/image-eg07.png")) {
        Ok(texture) => texture,
        Err(err)    => panic!("failed to convert surface: {}", err)
    };

    // Draw our image in the center of the window.
    // We need the foreground image's width to properly compute the position
    // of it's top left corner so that the image will be centered.
    let TextureQuery{ width: iw, height: ih, .. } = image.query();
    let x = (SCREEN_WIDTH - iw) as i32 / 2;
    let y = (SCREEN_HEIGHT - ih) as i32 /2;
    render_texture(&image, &mut renderer, x, y);

    renderer.present();

    let mut events = ctx.event_pump().unwrap();

    // Loop until we receive a QuitEvent or press escape.
    'event : loop {
        for event in events.wait_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::Window {win_event_id, ..} => {
                    match win_event_id {
                        WindowEventId::Exposed => renderer.present(),
                        _ => (),
                    }
                }
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'event
                    }
                }
                _               => continue
            }
        }
    }
}

