extern crate sdl2;

use std::io::timer::sleep;
use sdl2::rect::Point;

static WIDTH: int = 1024;
static HEIGHT: int = 768;
static COLDEPTH: int = 32;
static FPS: int = 60;
static DT: f64 = 0.2;
static EPS: f64 = 0.1;

struct Particle {
    pos : Vector,
    vel : Vector,
    mass: f64
}

struct Vector {
    x : f64,
    y : f64
}


fn force(p1: &Particle, p2: &Particle) -> Vector {
    let disp = diff(p1.pos, p2.pos);
    let dist = modls(disp);
    let f = p1.mass * p2.mass / (dist + EPS); // force magnitude
    Vector { x: -f*disp.x/dist, y: -f*disp.y/dist }
}

fn step(m: &mut Particle, M: &Particle) {
    // assume M doesn't change
    // update positions
    m.pos.x = m.pos.x + m.vel.x*DT;
    m.pos.y = m.pos.y + m.vel.y*DT;
    // update velocities
    let f = force(m, M);
    m.vel.x += f.x/m.mass*DT;
    m.vel.y += f.y/m.mass*DT;
}

fn dot(v1: Vector, v2: Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}

fn modls(v : Vector) -> f64 {
    (v.x * v.x + v.y* v.y).sqrt()
}

fn diff(v1 : Vector, v2: Vector) -> Vector {
    Vector { x: v1.x - v2.x, y: v1.y -v2.y }
}

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

fn animate() {
    sdl2::init(sdl2::InitVideo);

    let window = match sdl2::video::Window::new("Univ", sdl2::video::PosCentered, sdl2::video::PosCentered, WIDTH, HEIGHT, sdl2::video::OpenGL) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    let mut p1 = Particle { pos:Vector {x: 100., y: 0.}, vel:Vector {x:0.,y: 0.5}, mass:1. };
    let mut p2 = Particle { pos:Vector {x:-100., y: 0.}, vel:Vector {x:0.,y:-0.5}, mass:1. };

    let midx = (WIDTH/2) as f64;
    let midy = (HEIGHT/2) as f64;

    'main: loop {
        renderer.clear();
        renderer.draw_points(circle_points(p1.pos.x+midx, p1.pos.y+midy, p1.mass*10.).as_slice());
        renderer.draw_points(circle_points(p2.pos.x+midx, p2.pos.y+midy, p2.mass*10.).as_slice());
        renderer.present();
        step(&mut p1, &p2);
        step(&mut p2, &p1);
        match sdl2::event::poll_event() {
            sdl2::event::QuitEvent(_) => break 'main,
            sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                if key == sdl2::keycode::EscapeKey {
                    break 'main
                }
            }
            _ => {}
        }
    }
    sdl2::quit();
}

fn main() {
    animate()
}
/*
    let mut p1 = Particle { pos:Vector {x: 100., y: 0.}, vel:Vector {x:0.,y:0.}, mass:1. };
    let mut p2 = Particle { pos:Vector {x:-100., y: 0.}, vel:Vector {x:0.,y:0.}, mass:1. };

    let midx = (WIDTH/2) as f64;
    let midy = (HEIGHT/2) as f64;

    for _ in range(0,10) {
        step(&mut p1, &p2);
        step(&mut p2, &p1);
        println!("P1:{:?}    P2: {:?}", p1, p2);
    }
}

*/
