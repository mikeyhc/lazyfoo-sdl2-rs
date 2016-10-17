extern crate sdl2;

use sdl2::pixels::Color;

const WIDTH: u32  = 640;
const HEIGHT: u32 = 480;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();
    let window_builder = sdl_video.window("SDL Tutorial", WIDTH, HEIGHT);
    let mut window = window_builder.build().unwrap();
    let sdl_epump = sdl_context.event_pump().unwrap();
    {
        let mut surface = window.surface_mut(&sdl_epump).unwrap();
        surface.fill_rect(None, Color::RGB(0xFF, 0xFF, 0xFF)).unwrap();
    }
    window.update_surface().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    timer.delay(2000);
}
