extern crate sdl2;
extern crate rand;

use std::io::timer::sleep;
use sdl2::rect::Point;
use rand::random;

static WIDTH: int = 1024;
static HEIGHT: int = 768;
static COLDEPTH: int = 32;
static FPS: int = 60;
static DT: f64 = 0.1;
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

fn steppos(m: &mut Particle) {
    m.pos.x = m.pos.x + m.vel.x*DT;
    m.pos.y = m.pos.y + m.vel.y*DT;
}

fn stepvel(m1: &mut Particle, m2: &mut Particle) {
    let f = force(m1, m2);
    m1.vel.x += f.x/m1.mass*DT;
    m1.vel.y += f.y/m1.mass*DT;
    m2.vel.x -= f.x/m2.mass*DT;
    m2.vel.y -= f.y/m2.mass*DT;
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

fn circle_points(x: f64, y: f64, r:f64) -> ~[Point] {
    let mut points: ~[Point] = ~[];
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

    let mut particles: ~[Particle] = ~[];
    for x in range (0, 200) {
        let rnd1 = random::<int>() % 100;
        let rnd2 = random::<int>() % 100;
        particles.push(Particle { pos:Vector {x: rnd1 as f64, y: rnd2 as f64}, vel:Vector {x:0.,y:0.}, mass:1. });
    }
    let lenp = particles.len();

    let midx = (WIDTH/2) as f64;
    let midy = (HEIGHT/2) as f64;

    loop {
        renderer.clear();
        for p in particles.iter() {
            renderer.draw_points(circle_points(p.pos.x+midx, p.pos.y+midy, p.mass*10.).as_slice());
        }
        for i in range(0, lenp) {
            for j in range(i+1, lenp) {
                if i != j {
                    let f = force(&particles[i], &particles[j]);
                    particles[i].vel.x += f.x/particles[i].mass*DT;
                    particles[i].vel.y += f.y/particles[i].mass*DT;
                    particles[j].vel.x -= f.x/particles[j].mass*DT;
                    particles[j].vel.y -= f.y/particles[j].mass*DT;
                }
            }
        }
        for p in particles.mut_iter() {
            steppos(p);
        }
        renderer.present();
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
