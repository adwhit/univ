extern crate sdl;

use std::io::timer;

static WIDTH: int = 1024;
static HEIGHT: int = 768;
static COLDEPTH: int = 32;
static FPS: int = 60;


fn main() {
    sdl::init([sdl::InitVideo]);
    sdl::wm::set_caption("Univ", "What is this argument?");


    let screen = match sdl::video::set_video_mode(WIDTH, HEIGHT, COLDEPTH, 
                                                  [sdl::video::HWSurface], 
                                                  [sdl::video::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => fail!("failed to set video mode: {}", err) 
    };


    for x in range(0u16, 255) {
        let rect = make_rect(x, x);
        screen.fill_rect(Some(rect), sdl::video::RGB(x as u8,255-x as u8,255));
        screen.flip();
        timer::sleep(20);
    }

    sdl::quit();
}

fn make_rect(w: u16, h:u16) -> sdl::Rect {
    sdl::Rect {
        x: 2 * WIDTH as i16 / 10,
        y: 2 * HEIGHT as i16 / 10,
        w: w,
        h: h
    }
}

