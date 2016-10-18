extern crate sdl2;

use std::vec::Vec;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;

const WIDTH: u32  = 640;
const HEIGHT: u32 = 480;

fn init(sdl_context: &sdl2::Sdl) -> sdl2::video::Window {
    let sdl_video = sdl_context.video().unwrap();
    let wbuilder = sdl_video.window("SDL Tutorial", WIDTH, HEIGHT);
    wbuilder.build().unwrap()
}

fn load_media() -> Vec<Surface<'static>> {
    let mut ret = Vec::new();

    ret.push(load_surface("press.bmp"));
    ret.push(load_surface("up.bmp"));
    ret.push(load_surface("down.bmp"));
    ret.push(load_surface("left.bmp"));
    ret.push(load_surface("right.bmp"));
    ret
}

fn load_surface(path: &str) -> Surface<'static> {
    Surface::load_bmp(path).unwrap()
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut sdl_epump = sdl_context.event_pump().unwrap();
    let mut window = init(&sdl_context);
    let images = load_media();
    let mut current_surface = &images[0];
    {
        let mut surface = window.surface_mut(&sdl_epump).unwrap();
        current_surface.blit(None, &mut surface, None).unwrap();
    }
    window.update_surface().unwrap();

    loop {
        match sdl_epump.wait_event() {
            Event::Quit{timestamp: _}       => break,
            Event::KeyDown{keycode: k, .. } => {
                current_surface = match k {
                    Some(Keycode::Up)    => &images[1],
                    Some(Keycode::Down)  => &images[2],
                    Some(Keycode::Left)  => &images[3],
                    Some(Keycode::Right) => &images[4],
                    _                    => current_surface,
                };
                {
                    let mut surface = window.surface_mut(&sdl_epump).unwrap();
                    current_surface.blit(None, &mut surface, None).unwrap();
                }
                window.update_surface().unwrap();
            },
            _                          => ()
        }
    }
}
