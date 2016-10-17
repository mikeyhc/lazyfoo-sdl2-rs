extern crate sdl2;

use sdl2::surface::Surface;

const WIDTH: u32  = 640;
const HEIGHT: u32 = 480;

fn init(sdl_context: &sdl2::Sdl) -> sdl2::video::Window {
    let sdl_video = sdl_context.video().unwrap();
    let wbuilder = sdl_video.window("SDL Tutorial", WIDTH, HEIGHT);
    wbuilder.build().unwrap()
}

fn load_media() -> Surface<'static> {
    Surface::load_bmp("hello_world.bmp").unwrap()
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_epump = sdl_context.event_pump().unwrap();
    let mut window = init(&sdl_context);
    let image = load_media();
    {
        let mut surface = window.surface_mut(&sdl_epump).unwrap();
        image.blit(None, &mut surface, None).unwrap();
    }
    drop(image);
    window.update_surface().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    timer.delay(2000);
}
