extern crate serialize;
extern crate sdl2;
extern crate time;
extern crate getopts;
extern crate toml;

use sdl2::rect::Point;
use physics::Particle;
use config::{Display, Config, ConfigOpt};
use std::io::File;

mod physics;
mod barneshut;
mod config;


fn pcls2points(particles: &Vec<Particle>, display: Display) -> Vec<Point> {
    let midx = (display.width/2) as f64;
    let midy = (display.height/2) as f64;
    let mut arr: Vec<Point> = Vec::new();
    for p in particles.iter() {
        arr.push(Point {x: (p.pos.x + midx) as i32, y: (p.pos.y + midy) as i32 })
    }
    arr
}

fn init_particles(cfg: &Config) ->  (Vec<Particle>, fn(&mut Vec<Particle>)) {
    let mut particles : Vec<Particle> = Vec::new();
    for &gal in cfg.galaxies.iter() {
        let galaxy = physics::make_galaxy(gal);
        particles.push_all(galaxy.as_slice());
    };
    match cfg.sim {
        config::BarnesHut => return (particles, barneshut::stepsim),
        config::BarnesHutParallel => return (particles, barneshut::stepsim_par),
        config::Classical => return (particles, physics::stepsim),
    }
}

fn animate(mut particles: Vec<Particle>, stepfn: fn(&mut Vec<Particle>), display: Display ) {
    let renderer = get_renderer(display);
    renderer.clear();
    let mut framect = 0u;
    let starttime = time::precise_time_s();
    loop {
        renderer.clear();
        stepfn(&mut particles);
        let points = pcls2points(&particles, display);
        renderer.draw_points(points.as_slice());
        renderer.present();
        framect += 1;
        match sdl2::event::poll_event() {
            sdl2::event::QuitEvent(_) => break,
            sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                if key == sdl2::keycode::EscapeKey {
                    break
                }
            }
            _ => {}
        }
    }
    let endtime = time::precise_time_s();
    sdl2::quit();
    println!("Avg FPS: {}", framect as f64 / (endtime - starttime) as f64)
}

fn get_renderer(display: Display) -> sdl2::render::Renderer<sdl2::video::Window> {
    sdl2::render::Renderer::new_with_window(display.width as int, display.height as int, sdl2::video::FullscreenDesktop).unwrap()
}

// ******* Configuration ******* //

fn configure(path: &str) -> Config {
    let cfgstr = File::open(&Path::new(path)).read_to_str().unwrap();
    let cfgtbl = toml::Parser::new(cfgstr.as_slice()).parse().unwrap();
    let cfg: ConfigOpt = toml::decode(toml::Table(cfgtbl)).unwrap();

    let defaultstr = File::open(&Path::new("config/default.toml")).read_to_str().unwrap();
    let default_tbl = toml::Parser::new(defaultstr.as_slice()).parse().unwrap();
    let default: Config = toml::decode(toml::Table(default_tbl)).unwrap();
    default
}

fn opts() -> String {
    let args: Vec<String> = std::os::args().iter().map(|x| x.to_string()).collect(); 
    let opts = [
        getopts::optopt("c", "config", "Configuration file", "PATH")
        ];
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_str())
    };
    match matches.opt_str("c") {
        Some(c) => return c,
        None    => return String::from_str("config/default.toml")
    }
}

fn main() {
    let pathstr = opts();
    let cfg = configure(pathstr.as_slice());
    unsafe {barneshut::THRESH = cfg.threshold};
    unsafe {physics::DT = cfg.dt};
    let (particles, stepfn) = init_particles(&cfg);
    animate(particles, stepfn, cfg.display);
}
