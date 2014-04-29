extern crate sdl;

use std::io::timer::sleep;

static WIDTH: int = 1024;
static HEIGHT: int = 768;
static COLDEPTH: int = 32;
static FPS: int = 60;
static DT: f64 = 1.0;
static EPS: f64 = 0.001;

struct Particle {
    pos : Vector,
    vel : Vector,
    mass: f64
}

struct Vector {
    x : f64,
    y : f64
}


fn main() {
    sdl::init([sdl::InitVideo]);
    sdl::wm::set_caption("Univ", "What is this argument?");


    let screen = match sdl::video::set_video_mode(WIDTH, HEIGHT, COLDEPTH, 
                                                  [sdl::video::HWSurface], 
                                                  [sdl::video::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => fail!("failed to set video mode: {}", err) 
    };

    let mut m1 = Particle { Vector {x: 1, y: 1}, Vector {x:0,y:0}, mass:1 };
    let mut m2 = Particle { Vector {x:-1, y:-1}, Vector {x:0,y:0}, mass:1 };

    for x in range(0u16, 100) {
        screen.fill_rect(Some(rect), sdl::video::RGB(255,255,255));
        screen.flip();
        sleep(10);
    }

    sdl::quit();
}

fn force(p1: Particle, p2:Particle) -> Vector {
    let disp = diff(p1.pos, p2.pos);
    let dist = modls(disp);
    let f = p1.mass * p2.mass / (dist + EPS); // force magnitude
    Vector { x: f*dist/disp.x, y: f*dist/disp.y }
}

fn step(&mut m: Particle, &mut M: Particle) {
    // assume M doesn't change
    // update positions
    m.pos.x = m.pos.x + m.vel.x;
    m.pos.y = m.pos.y + m.vel.y;
    // update velocities
    let f = force(m, M);
    m.vel.x += f.x/m.mass*DT;
    m.vel.y += f.y/m.mass*DT;
}

fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}

fn modls(v : &Vector) -> f64 {
    (v.x.powi(2) + v.y.powi(2)).sqrt()
}

fn diff(v1 : &Vector, v2: &Vector) -> Vector {
    Vector { x: v1.x - v2.x, y: v1.y -v2.y }
}

