extern crate sdl2;
extern crate time;
extern crate toml = "rust-toml";

use sdl2::rect::Point;
use physics::{Particle, GalaxyCfg};

mod physics;
mod barneshut;

enum SimType {
    BarnesHut,
    BarnesHutParallel,
    Classical
}

struct Config {
    display :  Display,
    galaxies:  Vec<GalaxyCfg>,
    sim:       SimType,
    threshold: f64,
    dt       : f64

}

struct Display {
    width: uint,
    height: uint
}

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
        BarnesHut => return (particles, barneshut::stepsim),
        BarnesHutParallel => return (particles, barneshut::stepsim_par),
        Classical => return (particles, physics::stepsim),
    }
}

fn animate(mut particles: Vec<Particle>, stepfn: fn(&mut Vec<Particle>), display: Display ) {
    let renderer = get_renderer(display);
    renderer.clear();
    let mut framect = 0;
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

fn config_helper_int(root: &toml::Value, lookup: &str, default: i64) -> i64 {
    match root.lookup(lookup) {
        Some(v) => v.get_int().unwrap(),
        None    => default
    }
}

fn config_helper_float(root: &toml::Value, lookup: &str, default: f64) -> f64 {
    match root.lookup(lookup) {
        Some(v) => v.get_float().unwrap(),
        None    => default
    }
}

fn configure(path: &str) -> Config {
    let root = toml::parse_from_file(path).unwrap();
    let width = config_helper_int( &root, "display.screenwidth",  2048);
    let height = config_helper_int(&root, "display.screenheight", 1024);
    let simtype = match root.lookup("physics.simtype") {
        Some(v) => { 
            let sim = v.get_str().unwrap();
            if sim.equiv(&"barnes-hut") { BarnesHut }
            else if sim.equiv(&"classical") { Classical }
            else if sim.equiv(&"barnes-hut-parallel") { BarnesHutParallel }
            else { fail!("Error - {} not recognized", sim) }
        },
        None    => BarnesHut
    };
    Config { 
        display:  Display { width: width as uint, height: height as uint }, 
        galaxies: galaxy_configure(&root),
        sim:      simtype,
        threshold:config_helper_float(&root, "physics.threshold", 1.0),
        dt:       config_helper_float(&root, "physics.dt", 0.1)
    }
}

fn galaxy_configure(root: &toml::Value) -> Vec<GalaxyCfg> {
    let mut galaxies : Vec<GalaxyCfg> = Vec::new();
    let mut gal_ix = 0;
    loop {
        //nstars is only mandatory configuration argument
        let nbody = match root.lookup(format!("galaxy.{:d}.nbody", gal_ix).as_slice()) {
            Some(v) => v.get_int().unwrap() as uint,
            None    => break
        };
        // Galaxy shape and kinetics needs to be done separately and treat special options
        let shape = match root.lookup(format!("galaxy.{:d}.shape", gal_ix).as_slice()) {
            Some(v) => { 
                let shapestr = v.get_str().unwrap();
                if shapestr.equiv(&"random") { physics::RandomRadius }
                else if shapestr.equiv(&"concentric") { 
                    let nrings = config_helper_int(root, format!("galaxy.{:d}.nrings", gal_ix).as_slice(), 5) as uint;
                    physics::Concentric(nrings) 
                }
                else { fail!("Error - {} not recognized", shapestr) }
            },
            None    => physics::RandomRadius
        };
        let kinetics = match root.lookup(format!("galaxy.{:d}.kinetics", gal_ix).as_slice()) {
            Some(v) => { 
                let kinstr = v.get_str().unwrap();
                if kinstr.equiv(&"zero") { physics::ZeroVel }
                else if kinstr.equiv(&"random") { 
                    let minv = config_helper_float(root, format!("galaxy.{:d}.minv", gal_ix).as_slice(), 0.0);
                    let maxv = config_helper_float(root, format!("galaxy.{:d}.maxv", gal_ix).as_slice(), 10.0);
                    physics::RandomVel(minv, maxv) 
                }
                else if kinstr.equiv(&"circular") { physics::CircularOrbit }
                else { fail!("Error - {} not recognized", kinstr) }
            },
            None    => physics::CircularOrbit
        };
        galaxies.push( GalaxyCfg {
                posx : config_helper_float(root, format!("galaxy.{:d}.posx", gal_ix).as_slice(), 0.0),
                posy : config_helper_float(root, format!("galaxy.{:d}.posy", gal_ix).as_slice(), 0.0),
                velx : config_helper_float(root, format!("galaxy.{:d}.velx", gal_ix).as_slice(), 0.0),
                vely : config_helper_float(root, format!("galaxy.{:d}.vely", gal_ix).as_slice(), 0.0),
                radius : config_helper_float(root, format!("galaxy.{:d}.radius", gal_ix).as_slice(), 300.0),
                nbody : nbody,
                central_mass : config_helper_float(root, format!("galaxy.{:d}.centralmass", gal_ix).as_slice(), 1000.0),
                other_mass : config_helper_float(root, format!("galaxy.{:d}.othermass", gal_ix).as_slice(), 1.0),
                shape: shape,
                kinetics: kinetics });
        gal_ix += 1;
    }
    galaxies
}

fn main() {
    let cfg = configure("config/cfg.toml".as_slice());
    unsafe {barneshut::THRESH = cfg.threshold};
    unsafe {physics::DT = cfg.dt};
    let (particles, stepfn) = init_particles(&cfg);
    animate(particles, stepfn, cfg.display);
}
