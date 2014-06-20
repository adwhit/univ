extern crate sdl2;
extern crate toml = "rust-toml";

use sdl2::rect::Point;
use sdl2::pixels::{RGB, RGBA};
use std::rand;
use physics::{Particle, PhysVec, GalaxyCfg};

mod physics;
mod barneshut;

enum SimType {
    BarnesHut,
    Classical
}

struct Config {
    width:    uint,
    height:   uint,
    nbytes:   uint,
    galaxies: Vec<GalaxyCfg>,
    sim:      SimType
}

static WIDTH: uint = 2048;
static HEIGHT: uint = 1400;
static NBYTES: uint = 4;

fn circle_points(x: f64, y: f64, r:f64) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let rr = r as i32;
    let xr = x as i32;
    let yr = y as i32;
    for dy in range(0,rr) {
        let xlim = ((rr*rr - dy*dy) as f64).sqrt() as i32;
        for dx in range (0, xlim) {
            points.push(Point {x:xr + dx, y: yr + dy});
            points.push(Point {x:xr - dx, y: yr + dy});
            points.push(Point {x:xr + dx, y: yr - dy});
            points.push(Point {x:xr - dx, y: yr - dy});
        }
    }
    points
}

fn pcls2pixel(particles: &Vec<Particle>) -> Vec<u8> {
    let mut arr : Vec<u8> = Vec::from_fn(NBYTES*WIDTH*HEIGHT, |_| 0);
    let midx = (WIDTH/2) as f64;
    let midy = (HEIGHT/2) as f64;
    for p in particles.iter() {
        let xind = (p.pos.x + midx) as uint;
        let yind = (p.pos.y + midy) as uint;
        if xind < WIDTH && yind < HEIGHT {
            let ix = NBYTES * ((yind * WIDTH) + xind);
            *arr.get_mut(ix) = 0xff;
            *arr.get_mut(ix+1) = 0xff;
            *arr.get_mut(ix+2) = 0xff;
        }
    }
    arr
}

fn pcls2points(particles: &Vec<Particle>) -> Vec<Point> {
    let midx = (WIDTH/2) as f64;
    let midy = (HEIGHT/2) as f64;
    let mut arr: Vec<Point> = Vec::new();
    for p in particles.iter() {
        arr.push(Point {x: (p.pos.x + midx) as i32, y: (p.pos.y + midy) as i32 })
    }
    arr
}

fn init_particles(cfg: Config) ->  (Vec<Particle>, fn(&mut Vec<Particle>)) {
    let g = GalaxyCfg {
        posx: 0.0,
        posy: 0.0,
        velx: 0.0,
        vely: 0.0,
        radius: 300.,
        nstars: 500,
        shape:  physics::RandomRadius,
        kinetics: physics::ZeroVel,
        central_mass: 500.,
        other_mass: 1.
    };
    let galaxy1 = physics::make_galaxy(g);
    let mut particles : Vec<Particle> = Vec::new();
    particles.push_all(galaxy1.as_slice());
    match cfg.sim {
        BarnesHut => return (particles, barneshut::stepsim),
        Classical => return (particles, physics::stepsim),
    }
}

fn animate(particles: Vec<Particle>, stepfn: fn(&mut Vec<Particle>) ) {
    let lenp = particles.len();
    let renderer = get_renderer();
    let mut framect = 0;
    renderer.clear();
    loop {
        renderer.clear();
        stepfn(&mut particles);
        let points = pcls2points(&particles);
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
    sdl2::quit();
}

fn get_renderer() -> sdl2::render::Renderer<sdl2::video::Window> {
    sdl2::render::Renderer::new_with_window(WIDTH as int, HEIGHT as int, sdl2::video::FullscreenDesktop).unwrap()
}

fn config_helper_int(root: toml::Value, lookup: &str, default: i64) -> i64 {
    match root.lookup(lookup) {
        Some(v) => v.get_int().unwrap(),
        None    => default
    }
}

fn config_helper_float(root: toml::Value, lookup: &str, default: f64) -> f64 {
    match root.lookup(lookup) {
        Some(v) => v.get_float().unwrap(),
        None    => default
    }
}

fn configure(path: &str) -> Config {
    let root = toml::parse_from_file(path).unwrap();
    let width = config_helper_int( root, "global.screenwidth",  2048);
    let height = config_helper_int(root, "global.screenheight", 1024);
    let simtype = match root.lookup("physics.simtype") {
        Some(v) => { 
            let sim = v.get_str().unwrap();
            if sim.equiv(&"barnes-hut") { BarnesHut }
            else if sim.equiv(&"classical") { Classical }
            else { fail!("Error - {} not recognized", sim) }
        },
        None    => BarnesHut
    };
    Config { 
        width: width as uint, 
        height: height as uint, 
        nbytes: 4, 
        galaxies: galaxy_configure(root),
        sim: simtype }
}

fn galaxy_configure(root: toml::Value) -> Vec<GalaxyCfg> {
    let galaxies : Vec<GalaxyCfg> = Vec::new();
    let gal_ix = 0;
    loop {

        galaxies.push( GalaxyCfg {
                posx : config_helper_float(root, format!("galaxy.{:d}.posx", gal_ix), 0.0),
                posy : config_helper_float(root, format!("galaxy.{:d}.posy", gal_ix), 0.0),
                velx : config_helper_float(root, format!("galaxy.{:d}.velx", gal_ix), 0.0),
                vely : config_helper_float(root, format!("galaxy.{:d}.vely", gal_ix), 0.0),
                radius : config_helper_float(root, format!("galaxy.{:d}.radius", gal_ix), 300.0),
                nstars : config_helper_int(root, format!("galaxy.{:d}.nstars", gal_ix), 500),
                central_mass : config_helper_float(root, format!("galaxy.{:d}.central_mass", gal_ix), 1000.0),
                other_mass : config_helper_float(root, format!("galaxy.{:d}.other_mass", gal_ix), 1.0),
                shape: shape,
                kinetics: kinetics });
        gal_ix += 1;
        break;
    }
    galaxies
}

fn main() {
    let cfg = configure("config/cfg.toml".as_slice());
    println!("{}", cfg);
    //let (particles, stepfn) = init_particles(cfg);
    //animate(particles, stepfn);
}
