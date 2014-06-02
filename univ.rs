extern crate sdl2;
extern crate time;

use std::io::timer::sleep;
use sdl2::rect::Point;
use sdl2::pixels::{RGB, RGBA};
use std::rand;
use std::f64;

static WIDTH: uint = 2048;
static HEIGHT: uint = 1400;
static FPS: int = 60;
static DT: f64 = 0.05;
static EPS: f64 = 0.;
static NBYTES: uint = 4;

#[deriving(PartialEq)]
struct Particle {
    pos : Vector,
    vel : Vector,
    mass: f64
}

#[deriving(PartialEq)]
struct Vector {
    x : f64,
    y : f64
}

enum Galaxy {
    Random,
    Circular
}

impl Vector {
    fn add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    fn dot(&self, other: &Vector) -> f64 {
    self.x * other.x + self.y * other.y
    }

    fn modulus(&self) -> f64 {
        (self.x * self.x + self.y* self.y).sqrt()
    }

    //vector pointing from v1 towards v2
    fn diff(&self, v2: Vector) -> Vector {
        Vector { x: v2.x - self.x, y: v2.y -self.y }
    }

    fn angle(&self) -> f64 {
        let angle = (self.y/self.x).atan();
        if self.y > 0. && self.x < 0. {
            //upper left quadrant
            angle + f64::consts::PI
        } else if self.y < 0. && self.x < 0. {
            //lower left quadrant
            angle - f64::consts::PI
        } else {
            angle
        }
    }
}

impl Particle {
    fn kinetic_energy(&self) -> f64 {
        0.5 * ((self.vel.x * self.vel.x) + (self.vel.y * self.vel.y)) * self.mass
    }
}


//force is calculated as pointing from particle 1 towards particle 2
fn force(p1: &Particle, p2: &Particle) -> Vector {
    let disp = p1.pos.diff(p2.pos);
    let dist = disp.modulus() + EPS;
    let f = p1.mass * p2.mass / dist; // force magnitude
    Vector { x: f*disp.x/dist, y: f*disp.y/dist }
}

fn steppos(m: &mut Particle) {
    m.pos.x = m.pos.x + m.vel.x*DT;
    m.pos.y = m.pos.y + m.vel.y*DT;
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

fn spawn_circular_galaxy(max_radius: f64, num_stars: int) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    let pi =  f64::consts::PI;
    // n planets at radius r
    let ns = [0.5, 0.5];
    let rs = [0.5, 1.0];
    //let ns = [1.];
    //let rs = [1.];
    for (&nfrac, &rfrac) in ns.iter().zip(rs.iter()) {
        let n = (nfrac * num_stars as f64) as int;
        let r = rfrac * max_radius;
        for i in std::iter::range_inclusive(1, n) {
            let theta = (i as f64)/(n as f64)*2.0*pi;
            particles.push(Particle {pos:Vector {x: r*theta.cos(),   y: r*theta.sin()  }, 
                                     vel:Vector {x: 0., y: 0.},
                                     mass:1. });
        }
    }
    particles
}

fn spawn_random_galaxy(radius: f64, num_stars: int) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    for i in range(0,num_stars) {
        let theta = rand::random::<f64>()*360.;
        let r = rand::random::<f64>()*radius;
        let x =  r*theta.cos();
        let y =  r*theta.sin();
        particles.push(Particle {pos:Vector {x: x,  y: y },
                                 vel:Vector {x: 0., y: 0.},
                                 mass:1. });
    }
    particles
}

fn offset(particles: &mut Vec<Particle>, central_pcl: &Particle) {
    for p in particles.mut_iter() {
        p.pos.add(&central_pcl.pos);
        p.vel.add(&central_pcl.vel);
    }
}

fn make_galaxy(shape: Galaxy, central_pcl : Particle, radius: f64, num_stars: int) -> Vec<Particle> {
    let mut particles = match shape {
        Random => spawn_random_galaxy(radius, num_stars),
        Circular => spawn_circular_galaxy(radius, num_stars)
    };
    init_velocity(&mut particles, central_pcl.mass);
    offset(&mut particles, &central_pcl);
    particles.push(central_pcl);
    particles
}

fn init_velocity(particles: &mut Vec<Particle>, central_mass: f64) {
    let mut vels : Vec<Vector> = Vec::new();

    // need to make dummy since we are initialising centred on zero
    let dummy_central_pcl = Particle { pos: Vector {x:0., y:0.},
                                       vel: Vector {x:0., y:0.},
                                       mass: central_mass };
    for p in particles.iter() {
        let mut forcev = Vector {x : 0., y: 0.};
        for q in particles.iter() {
            if q != p {
                forcev.add(&force(p, q))
            }
        }
        forcev.add(&force(p, &dummy_central_pcl));
        let theta = p.pos.angle();
        let speed = (forcev.modulus()*p.pos.modulus()/p.mass).sqrt();
        if theta.is_nan() {
            let v = Vector {x: 0., y: 0.};
            vels.push(v);
        } else {
            let v = Vector {x: speed*theta.sin(), y: -speed*theta.cos()};
            vels.push(v);
        }
    }
    for (p,&v) in particles.mut_iter().zip(vels.iter()) {
        p.vel  = v;
    }
}

//Calculates particle with equivalent centre of mass and total mass
fn centre_of_mass(particles: &Vec<Particle>) -> Particle {
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
    let renderer = get_renderer();

    let centre1 = Particle { pos: Vector {x: 0., y:0.},
                         vel: Vector {x: 0., y:0.},
                         mass: 1000.};
    let centre2 = Particle { pos: Vector {x: 0., y:0.},
                         vel: Vector {x: 10., y:10.},
                         mass: 1.};
    let mut particles = make_galaxy(Circular, centre1, 400., 1000);
    //let galaxy2 = make_galaxy(Circular, centre2, 400., 1000);
    
    print_state(&particles);

    let lenp = particles.len();
    let mut framect = 0;
    let pixels : ~[u8] = ~[0,..NBYTES*WIDTH*HEIGHT];

    renderer.clear();
    loop {
        renderer.clear();
        stepsim(&mut particles, lenp);
        let points = pcls2points(&particles);
        renderer.draw_points(points.as_slice());
        if framect % 10 ==0 {
            //println!("{} : {}", time::now().ctime(), framect);
        }
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
    print_state(&particles);
}

fn get_renderer() -> sdl2::render::Renderer<sdl2::video::Window> {
    sdl2::render::Renderer::new_with_window(WIDTH as int, HEIGHT as int, sdl2::video::FullscreenDesktop).unwrap()
}

fn total_ke(particles: &Vec<Particle>) -> f64 {
    let mut ke = 0.;
    for p in particles.iter() {
        ke += p.kinetic_energy()
    }
    ke
}

fn print_state(particles: &Vec<Particle>) {
    for &p in particles.iter() {
        println!("x: {:0.2f}\ty: {:0.2f}\t xv: {:0.2f}\t yv: {:0.2f}\tr: {:0.2f}\tv {:0.2f}", 
                p.pos.x, p.pos.y, p.vel.x, p.vel.y, p.pos.modulus(), p.vel.modulus())
    }
    println!("KE: {:0.2f}", total_ke(particles));
}


fn main() {
    animate();
}
