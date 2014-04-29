extern crate sdl2;

use std::io::timer::sleep;
use sdl2::rect::Point;

static WIDTH: int = 1024;
static HEIGHT: int = 768;
static COLDEPTH: int = 32;
static FPS: int = 60;
static DT: f64 = 1.0;
static EPS: f64 = 0.001;
static BPP: int = 4; 

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
    Vector { x: f*dist/disp.x, y: f*dist/disp.y }
}

fn step(m: &mut Particle, M: &mut Particle) {
    // assume M doesn't change
    // update positions
    m.pos.x = m.pos.x + m.vel.x;
    m.pos.y = m.pos.y + m.vel.y;
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
    let mut dy = 1;
    let rr = r as i32;
    let xr = x as i32;
    let yr = y as i32;
    while dy <= rr {
        let xlim = ((rr - dy*dy) as f64).sqrt() as i32;
        for dx in range (0, xlim) {
            points.push(Point {x:xr + dx, y: yr + dy});
            points.push(Point {x:xr - dx, y: yr + dy});
            points.push(Point {x:xr + dx, y: yr - dy});
            points.push(Point {x:xr - dx, y: yr - dy});
        }
    }
    points
}

fn main() {
    sdl2::init(sdl2::InitVideo);

    let window = match sdl2::video::Window::new("Univ", sdl2::video::PosCentered, sdl2::video::PosCentered, WIDTH, HEIGHT, sdl2::video::OpenGL) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    renderer.clear();
    renderer.draw_points(circle_points(0.,0.,10.).as_slice());
    renderer.present();
    sleep(1000);

    sdl2::quit();

    //let mut m1 = Particle { pos:Vector {x: 1, y: 1}, vel:Vector {x:0,y:0}, mass:1 };
    //let mut m2 = Particle { pos:Vector {x:-1, y:-1}, vel:Vector {x:0,y:0}, mass:1 };

}

