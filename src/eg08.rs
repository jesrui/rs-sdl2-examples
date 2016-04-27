/*
This is just a translation to Rust of lesson 5 of Will Usher's SDL
tutorial. See
http://www.willusher.io/sdl2%20tutorials/2013/08/27/lesson-5-clipping-sprite-sheets

In this example we'll see how to select subsections of a sprite sheet.

 */

extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::event::{Event,WindowEventId};
use sdl2::rect::{Rect,Point};
use sdl2::keyboard::Keycode;

use sdl2_image::{LoadTexture, INIT_PNG};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

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

    let window  = match video_ctx.window("eg08", SCREEN_WIDTH, SCREEN_HEIGHT)
                .position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    renderer.clear();

    let image = match renderer.load_texture(&Path::new("res/image-eg08.png")) {
        Ok(texture) => texture,
        Err(err)    => panic!("failed to convert surface: {}", err)
    };

    // Setup the clips for our image.
    let iw = 100;
    let ih = 100;
    let clips = {
        let mut cs = [Rect::new(0, 0, iw, ih); 4];
        for (i, clip) in cs.iter_mut().enumerate() {
            clip.set_x(i as i32 / 2 * iw as i32);
            clip.set_y(i as i32 % 2 * iw as i32);
        }
        cs
    };

    let mut use_clip;

    // Draw our image in the center of the window.
    // We need the foreground image's width to properly compute the position
    // of it's top left corner so that the image will be centered.
    let dst = Rect::from_center(Point::new(SCREEN_WIDTH as i32 / 2,
                                           SCREEN_HEIGHT as i32 / 2),
                                iw, ih);

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
                    match keycode {
                        Keycode::Escape => break 'event,
                        Keycode::Num1 => use_clip = 0,
                        Keycode::Num2 => use_clip = 1,
                        Keycode::Num3 => use_clip = 2,
                        Keycode::Num4 => use_clip = 3,
                        _ => continue,
                    }
                    renderer.clear();
                    renderer.copy(&image, Some(clips[use_clip]), Some(dst));
                    renderer.present();
                }
                _               => continue
            }
        }
    }
}

