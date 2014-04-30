extern crate sdl2;
extern crate rand;

use std::io::timer::sleep;
use sdl2::rect::Point;
use sdl2::pixels::{RGB, RGBA};
use rand::random;
use std::f64;

static WIDTH: int = 1024;
static HEIGHT: int = 768;
static COLDEPTH: int = 32;
static FPS: int = 60;
static DT: f64 = 0.05;
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

fn make_galaxy() -> ~[Particle] {
    let mut particles: ~[Particle] = ~[];
    let pi =  f64::consts::PI;
    let ns = [20,  50,   100,  150,  200];
    let rs = [50., 100., 200., 250., 300.];
    let velweight = 400.0;

    for (&n, &r) in ns.iter().zip(rs.iter()) {
        for i in std::iter::range_inclusive(1, n) {
            let theta = (i as f64)/(n as f64)*2.0*pi;
            let vel = velweight*(1.0/r).sqrt();
            particles.push(Particle {pos:Vector {x: r*theta.cos(),   y: r*theta.sin()  }, 
                                     vel:Vector {x: vel*theta.sin(), y: -vel*theta.cos()},
                                     mass:1. });
        }
    }
    //central particle
    particles.push(Particle {pos:Vector {x: 0., y: 0.  }, 
                             vel:Vector {x: 0., y: 0.}, mass:1000. });
    particles
}

fn initwindow() -> ~sdl2::render::Renderer {
    sdl2::init(sdl2::InitVideo);
    let window = match sdl2::video::Window::new("Univ", sdl2::video::PosCentered, sdl2::video::PosCentered, WIDTH, HEIGHT, sdl2::video::OpenGL) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };
    renderer
}

fn stepvel(force: Vector, p: &mut Particle, sense:bool) {
    if sense {
        p.vel.x += force.x/p.mass*DT;
        p.vel.y += force.y/p.mass*DT;
    } else {
        p.vel.x -= force.x/p.mass*DT;
        p.vel.y -= force.y/p.mass*DT;
    }
}


fn animate() {

    let renderer = initwindow();
    let mut particles = make_galaxy();

    let lenp = particles.len();
    let midx = (WIDTH/2) as f64;
    let midy = (HEIGHT/2) as f64;
    let m2sz = 5.0;


    renderer.clear();
    loop {
        renderer.set_draw_color(RGB(0,0,0));
        renderer.clear();
        for (ix,p) in particles.iter().enumerate() {
            renderer.set_draw_color(RGB(255,255,255));
            renderer.draw_points(circle_points(p.pos.x+midx, p.pos.y+midy, (m2sz*p.mass.powf(&0.333)).sqrt()).as_slice());
        }
        for i in range(0, lenp) {
            for j in range(i+1, lenp) {
                if i != j {
                    let mut f = force(&particles[i], &particles[j]);
                    stepvel(f, &mut particles[i], true);
                    stepvel(f, &mut particles[j], false);
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
