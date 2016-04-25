/*
This is just a translation to Rust of lesson 2 of Will Usher's SDL
tutorial. See
http://www.willusher.io/sdl2%20tutorials/2013/08/17/lesson-2-dont-put-everything-in-main

In this example, we'll show first how to render a tiled background and then an
image on top of it.

 */

extern crate sdl2;

use std::path::Path;
use std::fmt;
use std::fmt::Debug;

use sdl2::event::{Event,WindowEventId};
use sdl2::render::{Renderer,Texture,TextureValueError};
use sdl2::rect::{Rect};
use sdl2::surface::{Surface};
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

// errors from Result's of sdl2 types are handled like explained in:
// http://rustbyexample.com/error/reenter_try.html and
// http://rustbyexample.com/error/option_with_result/enter_try.html
type Result<T> = std::result::Result<T, EgError>;

#[derive(Debug)]
enum EgError {
    LoadBMPError(String),
    TextureError(TextureValueError),
}

impl From<TextureValueError> for EgError {
    fn from(err: TextureValueError) -> EgError {
        EgError::TextureError(err)
    }
}

impl fmt::Display for EgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EgError::TextureError(ref err) => err.fmt(f),
            EgError::LoadBMPError(ref s) => write!(f, "LoadBMPError: {}", s),
        }
    }
}

/*
 * Loads a BMP image into a texture on the rendering device
 * @param bmp_fname The BMP image file to load
 * @param renderer The renderer to load the texture onto
 * @return the loaded texture, or an EgError if something went wrong.
 */
fn load_texture(bmp_fname: &str, renderer: &mut Renderer) -> Result<Texture> {
    // Load a surface.
    // Surfaces live in system RAM, so they aren't ideal for performance.
    let surface = try!(Surface::load_bmp(&Path::new(bmp_fname))
        .map_err(|err| EgError::LoadBMPError(err))
    );

    // Convert a surface to a texture.
    // Textures can be used more efficiently by the GPU. (If one is available.)
    let texture = try!(renderer.create_texture_from_surface(&surface));

    Ok(texture)
}

/*
 * Draw an SDL_Texture to an SDL Renderer at position x, y, preserving
 * the texture's width and height
 * @param texture The source texture we want to draw
 * @param renderer The renderer we want to draw to
 * @param x The x coordinate to draw to
 * @param y The y coordinate to draw to
 */
fn render_texture(texture: &Texture, renderer: &mut Renderer, x: i32, y: i32) {
    let qry = texture.query();
    let dst = Rect::new(x, y, qry.width, qry.height);
    renderer.copy(&texture, None, Some(dst));
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx.window("eg06", SCREEN_WIDTH, SCREEN_HEIGHT)
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
    let background = match load_texture("res/background.bmp", &mut renderer) {
        Ok(texture) => texture,
        Err(err)    => panic!("failed to convert surface: {}", err)
    };

    // Display the tiled background.
    // Get the width and height from the texture so we know how much to move x,y by
    // to tile it correctly.
    let qry = background.query();
    let bw = qry.width as i32;
    let bh = qry.height as i32;
    render_texture(&background, &mut renderer, 0,  0);
    render_texture(&background, &mut renderer, bw, 0);
    render_texture(&background, &mut renderer, 0,  bh);
    render_texture(&background, &mut renderer, bw, bh);

    let image = match load_texture("res/image.bmp", &mut renderer) {
        Ok(texture) => texture,
        Err(err)    => panic!("failed to convert surface: {}", err)
    };

    // Draw our image in the center of the window.
    // We need the foreground image's width to properly compute the position
    // of it's top left corner so that the image will be centered.
    let qry = image.query();
    let iw = qry.width as i32;
    let ih = qry.height as i32;
    let x = (SCREEN_WIDTH as i32)/2 - iw/2;
    let y = (SCREEN_HEIGHT as i32)/2 - ih/2;
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

