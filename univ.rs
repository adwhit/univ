extern crate sdl2;
extern crate rand;

use std::io::timer::sleep;
use sdl2::rect::Point;
use sdl2::pixels::{RGB, RGBA};
use rand::random;
use std::f64;

static WIDTH: uint = 1024;
static HEIGHT: uint = 768;
static FPS: int = 60;
static DT: f64 = 0.1;
static EPS: f64 = 0.1;
static NBYTES: uint = 4;

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

fn stepvel(force: Vector, p: &mut Particle, sense:bool) {
    if sense {
        p.vel.x += force.x/p.mass*DT;
        p.vel.y += force.y/p.mass*DT;
    } else {
        p.vel.x -= force.x/p.mass*DT;
        p.vel.y -= force.y/p.mass*DT;
    }
}

fn stepsim(particles: &mut [Particle], lenp: uint) {
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
}

fn pcls2pixel(particles: &[Particle]) -> ~[u8] {
    let mut arr : ~[u8] = ~[0,..NBYTES*WIDTH*HEIGHT];
    let midx = (WIDTH/2) as f64;
    let midy = (HEIGHT/2) as f64;
    for p in particles.iter() {
        let xind = (p.pos.x + midx) as uint;
        let yind = (p.pos.y + midy) as uint;
        if xind < WIDTH && yind < HEIGHT {
            let ix = NBYTES * ((yind * WIDTH) + xind);
            arr[ix] = 0xff;
            arr[ix+1] = 0xff;
            arr[ix+2] = 0xff;
        }
    }
    arr
}

fn animate() {
    let renderer = get_renderer();
    let mut particles = make_galaxy();
    let lenp = particles.len();
    let mask = renderer.create_texture(sdl2::pixels::RGB888,
                                       sdl2::render::AccessStreaming,WIDTH as int,HEIGHT as int).unwrap();
    let newframe = renderer.create_texture(sdl2::pixels::RGB888,
                                       sdl2::render::AccessStreaming,WIDTH as int,HEIGHT as int).unwrap();
    mask.set_blend_mode(sdl2::render::BlendBlend);
    mask.set_alpha_mod(10);

    renderer.clear();
    loop {
        let pixels = pcls2pixel(particles);
        mask.update(None, pixels, (WIDTH * NBYTES) as int);
        renderer.copy(mask,None,None);
        stepsim(particles, lenp);
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

fn get_renderer() -> ~sdl2::render::Renderer {
    sdl2::render::Renderer::new_with_window(1000, 1000, sdl2::video::FullscreenDesktop).unwrap()
}


fn bitmappy() {
    let renderer = get_renderer();
    renderer.clear();
    let bmp = match sdl2::surface::Surface::from_bmp(&std::path::Path::new("lib/LAND3.BMP")) {
        Ok(bmap) =>  bmap,
        Err(e)   => fail!(e)
    };
    let tex = match renderer.create_texture_from_surface(bmp) {
        Ok(t) => t,
        Err(e) => fail!(e)
    };
    renderer.copy(tex, None, Some(sdl2::rect::Rect::new(100,100,100,100)));
    renderer.present();
    loop {
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
}

fn main() {
    //bitmappy();
    animate();
}
