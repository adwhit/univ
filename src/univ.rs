#![feature(core, collections)]

extern crate rustc_serialize;
extern crate sdl2;
extern crate time;
extern crate getopts;
extern crate toml;
extern crate rand;
extern crate deque;

use sdl2::rect::Point;
use physics::Particle;
use config::{Display, Config, ConfigOpt};
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
    for gal in &cfg.galaxies {
        let galaxy = physics::make_galaxy(gal.clone());
        particles.push_all(&galaxy);
    };
    match cfg.sim {
        config::SimType::BarnesHut => return (particles, barneshut::stepsim),
        config::SimType::BarnesHutParallel => return (particles, barneshut::stepsim_par),
        config::SimType::Classical => return (particles, physics::stepsim),
    }
}

fn animate(mut particles: Vec<Particle>, stepfn: fn(&mut Vec<Particle>), display: Display ) {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    let mut renderer = get_renderer(&sdl_context, display);
    let mut drawer = renderer.drawer();
    let mut framect = 0;
    let starttime = time::precise_time_s();
    let mut event_pump = sdl_context.event_pump();
    'outer: loop {
        drawer.clear();
        stepfn(&mut particles);
        let points = pcls2points(&particles, display);
        drawer.draw_points(&points);
        drawer.present();
        framect += 1;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{..} => break 'outer,
                sdl2::event::Event::KeyDown { keycode:key, .. } => {
                    if key == sdl2::keycode::KeyCode::Escape {
                        break 'outer
                    }
                }
                _ => {}
            }
        }
    }
    let endtime = time::precise_time_s();
    println!("Avg FPS: {}", framect as f64 / (endtime - starttime) as f64)
}

fn get_renderer<'a>(_sdl: &sdl2::Sdl, display: Display) -> sdl2::render::Renderer<'a> {
    sdl2::render::Renderer::new_with_window(_sdl, display.width, display.height, sdl2::video::FULLSCREEN).unwrap()
}

// ******* Configuration ******* //

fn configure(path: &str) -> Config {
    let mut cfgstr = String::new();
    File::open(&Path::new(path)).unwrap().read_to_string(&mut cfgstr);
    let cfgtbl = toml::Parser::new(&cfgstr).parse().unwrap();
    let cfg: ConfigOpt = toml::decode(toml::Value::Table(cfgtbl)).unwrap();

    let mut defaultstr = String::new();
    File::open(&Path::new("config/default.toml")).unwrap().read_to_string(&mut defaultstr);
    let default_tbl = toml::Parser::new(&defaultstr).parse().unwrap();
    let default: Config = toml::decode(toml::Value::Table(default_tbl)).unwrap();
    println!("{:?}", default);
    default
}

fn opts() -> String {
    let args: Vec<String> = std::env::args().map(|x| x.to_string()).collect(); 
    let mut opts = getopts::Options::new();
    opts.optopt("c", "config", "Configuration file", "PATH");
    let matches = match opts.parse(args.tail()) {
        Ok(m) => m,
        Err(f) => panic!("{}", f)
    };
    match matches.opt_str("c") {
        Some(c) => return c,
        None    => return String::from_str("config/default.toml")
    }
}

fn main() {
    let pathstr = opts();
    let cfg = configure(&pathstr);
    unsafe {barneshut::THRESH = cfg.threshold};
    unsafe {physics::DT = cfg.dt};
    let (particles, stepfn) = init_particles(&cfg);
    animate(particles, stepfn, cfg.display);
}
