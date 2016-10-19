extern crate sdl2;

use std::vec::Vec;
use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormat;
use sdl2::rect::Rect;

const WIDTH: u32  = 640;
const HEIGHT: u32 = 480;

fn init(sdl_context: &sdl2::Sdl) -> sdl2::video::Window {
    let sdl_video = sdl_context.video().unwrap();
    let wbuilder = sdl_video.window("SDL Tutorial", WIDTH, HEIGHT);
    wbuilder.build().unwrap()
}

fn load_media(format: &PixelFormat) -> Vec<Surface<'static>> {
    let mut ret = Vec::new();

    ret.push(load_surface("stretch.bmp", format));
    ret
}

fn load_surface(path: &str, format: &PixelFormat) -> Surface<'static> {
    let loaded = Surface::load_bmp(path).unwrap();
    loaded.convert(format).unwrap()
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut sdl_epump = sdl_context.event_pump().unwrap();
    let mut window = init(&sdl_context);
    let format = window.surface(&sdl_epump).unwrap().pixel_format();
    let images = load_media(&format);
    let current_surface = &images[0];
    {
        let mut surface = window.surface_mut(&sdl_epump).unwrap();
        let rect = Some(Rect::new(0, 0, WIDTH, HEIGHT));
        current_surface.blit_scaled(None, &mut surface, rect).unwrap();
    }
    window.update_surface().unwrap();

    loop {
        match sdl_epump.wait_event() {
            Event::Quit{timestamp: _} => break,
            _                         => (),
        }
    }
}
