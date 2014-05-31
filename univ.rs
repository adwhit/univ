extern crate sdl2;
extern crate rand;

use std::io::timer::sleep;
use sdl2::rect::Point;
use sdl2::pixels::{RGB, RGBA};
use rand::random;
use std::f64;

static WIDTH: uint = 2048;
static HEIGHT: uint = 1400;
static FPS: int = 60;
static DT: f64 = 0.1;
static EPS: f64 = 0.;
static NBYTES: uint = 4;

#[deriving(Eq)]
struct Particle {
    pos : Vector,
    vel : Vector,
    mass: f64
}

#[deriving(Eq)]
struct Vector {
    x : f64,
    y : f64
}

impl Vector {
    fn add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}


//force is calculated as pointing from particle 1 towards particle 2
fn force(p1: &Particle, p2: &Particle) -> Vector {
    let disp = diff(p1.pos, p2.pos);
    let dist = modls(disp) + EPS;
    let f = p1.mass * p2.mass / dist; // force magnitude
    Vector { x: f*disp.x/dist, y: f*disp.y/dist }
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

//vector pointing from v1 towards v2
fn diff(v1 : Vector, v2: Vector) -> Vector {
    Vector { x: v2.x - v1.x, y: v2.y -v1.y }
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

fn make_galaxy() -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    let pi =  f64::consts::PI;
    // n planets at radius r
    //let ns = [20,  50,   100,  150,  200];
    //let rs = [50., 100., 200., 250., 300.];
    let ns = [50];
    let rs = [100.];
    //parameter to weight velocities
    let velweight = 400.0;

    for (&n, &r) in ns.iter().zip(rs.iter()) {
        for i in std::iter::range_inclusive(1, n) {
            let theta = (i as f64)/(n as f64)*2.0*pi;
            //XXX want to change to make universe stable
            let vel = velweight*(1.0/r).sqrt();
            particles.push(Particle {pos:Vector {x: r*theta.cos(),   y: r*theta.sin()  }, 
                                     //vel:Vector {x: 0., y: 0.},
                                     vel:Vector {x: vel*theta.sin(), y: -vel*theta.cos()},
                                     mass:1. });
        }
    }
    //central particle
    //particles.push(Particle {pos:Vector {x: 0., y: 0.  }, 
    //                         vel:Vector {x: 0., y: 0.}, mass:1000. });
    particles
}

fn init_velocity(particles: &mut Vec<Particle>) {
    let mut vels : Vec<Vector> = Vec::new();
    let weight = 1.;
    for p in particles.iter() {
        let mut tot_force = Vector {x : 0., y: 0.};
        for q in particles.iter() {
            if q != p {
                tot_force.add(&force(p, q))
            }
        }
        let r = modls(p.pos);

        let mut v = Vector { x: (tot_force.y.abs()*r/p.mass).sqrt()*weight,
                         y: (tot_force.x.abs()*r/p.mass).sqrt()*weight} ;
        if tot_force.x < 0. { v.y *= -1. };
        if tot_force.y > 0. { v.x *= -1. };
        vels.push(v);
        println!("pos x:{} fx:{} vel y:{} pos y:{} fy:{} vel x:{} tot_v:{}", p.pos.x, tot_force.x,v.y, p.pos.y, tot_force.y,v.x,modls(v));
    }
    for (p,&v) in particles.mut_iter().zip(vels.iter()) {
        p.vel  = v;
    }
}

//Calculates particle with equivalent centre of mass and total mass
fn centre_of_mass(particles: Vec<Particle>) -> Particle {
    //position
    let mut rx = 0.;
    let mut ry = 0.;
    //momentum
    let mut px = 0.;
    let mut py = 0.;
    //mass
    let mut m = 0.;
    for p in particles.iter() {
        rx += p.pos.x*p.mass;
        ry += p.pos.y*p.mass;
        px += p.vel.x*p.mass;
        py += p.vel.y*p.mass;
        m += p.mass;
    }
    return Particle {pos: Vector {x: rx/m, y: ry/m},
                     vel: Vector {x: px/m, y: py/m},
                     mass: m}
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

fn stepsim(particles: &mut Vec<Particle>, lenp: uint) {
    for i in range(0, lenp) {
        for j in range(i+1, lenp) {
            if i != j {
                let mut f = force(particles.get(i), particles.get(j));
                stepvel(f, particles.get_mut(i), true);
                stepvel(f, particles.get_mut(j), false);
            }
        }
    }
    for p in particles.mut_iter() {
        steppos(p);
    }
}

fn pcls2pixel(particles: &Vec<Particle>) -> ~[u8] {
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

fn pcls2points(particles: &Vec<Particle>) -> Vec<Point> {
    let midx = (WIDTH/2) as f64;
    let midy = (HEIGHT/2) as f64;
    let mut arr: Vec<Point> = Vec::new();
    for p in particles.iter() {
        arr.push(Point {x: (p.pos.x + midx) as i32, y: (p.pos.y + midy) as i32 })
    }
    arr
}

fn animate() {
    //This is pretty mangled - half way through trying to create tails
    //by combining textures with alpha-transparancy
    let renderer = get_renderer();
    let mut particles = make_galaxy();
    init_velocity(&mut particles);
    let lenp = particles.len();
    //create mask
    let mask = renderer.create_texture(sdl2::pixels::RGB888,
                                       sdl2::render::AccessStreaming,WIDTH as int,HEIGHT as int).unwrap();
    let base = renderer.create_texture(sdl2::pixels::RGB888,
                                       sdl2::render::AccessStreaming,WIDTH as int,HEIGHT as int).unwrap();
    let pixels : ~[u8] = ~[0,..NBYTES*WIDTH*HEIGHT];
    mask.update(None, pixels, (WIDTH * NBYTES) as int);
    mask.set_blend_mode(sdl2::render::BlendNone);
    mask.set_alpha_mod(100);

    renderer.clear();
    loop {
        renderer.clear();
        stepsim(&mut particles, lenp);
        let points = pcls2points(&particles);
        //mask.update(None, pcls2pixel(&particles), (WIDTH*NBYTES) as int);
        //renderer.copy(&mask,None,None);
        renderer.draw_points(points.as_slice());
        //renderer.draw_line(Point::new(1,2),Point::new(30, 50));
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

fn get_renderer() -> sdl2::render::Renderer<sdl2::video::Window> {
    sdl2::render::Renderer::new_with_window(WIDTH as int, HEIGHT as int, sdl2::video::FullscreenDesktop).unwrap()
}


fn bitmappy() {
    let renderer = get_renderer();
    renderer.clear();
    let bmp = match sdl2::surface::Surface::from_bmp(&std::path::Path::new("lib/LAND3.BMP")) {
        Ok(bmap) =>  bmap,
        Err(e)   => fail!(e)
    };
    let tex = match renderer.create_texture_from_surface(&bmp) {
        Ok(t) => t,
        Err(e) => fail!(e)
    };
    renderer.copy(&tex, None, Some(sdl2::rect::Rect::new(100,100,100,100)));
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
