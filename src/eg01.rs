extern crate sdl2;

fn main() {
    // start sdl2

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut timer = ctx.timer().unwrap();

    // Create a window

    let mut window = match video_ctx.window("eg01", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Display the window for 3 seconds

    window.show();
    timer.delay(3000);
}

