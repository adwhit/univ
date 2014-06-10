use std::{f64, iter, rand};

static DT: f64 = 0.05;
static EPS: f64 = 0.;

#[deriving(PartialEq)]
#[deriving(Clone)]
pub struct Particle {
    pub pos : PhysVec,
    pub vel : PhysVec,
    pub mass: f64
}

#[deriving(PartialEq)]
#[deriving(Clone)]
pub struct PhysVec {
    pub x : f64,
    pub y : f64
}

pub enum Galaxy {
    Random,
    Circular
}

impl PhysVec {
    fn add(&mut self, other: &PhysVec) {
        self.x += other.x;
        self.y += other.y;
    }

    fn dot(&self, other: &PhysVec) -> f64 {
    self.x * other.x + self.y * other.y
    }

    fn modulus(&self) -> f64 {
        (self.x * self.x + self.y* self.y).sqrt()
    }

    //vector pointing from v1 towards v2
    fn diff(&self, v2: PhysVec) -> PhysVec {
        PhysVec { x: v2.x - self.x, y: v2.y -self.y }
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

    fn steppos(&mut self) {
        self.pos.x = self.pos.x + self.vel.x*DT;
        self.pos.y = self.pos.y + self.vel.y*DT;
    }

}


//force is calculated as pointing from particle 1 towards particle 2
fn force(p1: &Particle, p2: &Particle) -> PhysVec {
    let disp = p1.pos.diff(p2.pos);
    let dist = disp.modulus() + EPS;
    let f = p1.mass * p2.mass / dist; // force magnitude
    PhysVec { x: f*disp.x/dist, y: f*disp.y/dist }
}


fn spawn_circular_galaxy(max_radius: f64, num_stars: int) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    let pi =  f64::consts::PI;
    // n planets at radius r
    let ns = [0.1,0.2,0.3,0.4];
    let rs = [0.2,0.5,0.7,1.0];
    //let ns = [1.];
    //let rs = [1.];
    for (&nfrac, &rfrac) in ns.iter().zip(rs.iter()) {
        let n = (nfrac * num_stars as f64) as int;
        let r = rfrac * max_radius;
        for i in iter::range_inclusive(1, n) {
            let theta = (i as f64)/(n as f64)*2.0*pi;
            particles.push(Particle {pos:PhysVec {x: r*theta.cos(),   y: r*theta.sin()  }, 
                                     vel:PhysVec {x: 0., y: 0.},
                                     mass:1. });
        }
    }
    particles
}

fn spawn_random_galaxy(radius: f64, num_stars: int) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    for _ in range(0,num_stars) {
        let theta = rand::random::<f64>()*360.;
        let r = rand::random::<f64>()*radius;
        let x =  r*theta.cos();
        let y =  r*theta.sin();
        particles.push(Particle {pos:PhysVec {x: x,  y: y },
                                 vel:PhysVec {x: 0., y: 0.},
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

pub fn make_galaxy(shape: Galaxy, central_pcl : Particle, radius: f64, num_stars: int) -> Vec<Particle> {
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
    let mut vels : Vec<PhysVec> = Vec::new();

    // need to make dummy since we are initialising centred on zero
    let dummy_central_pcl = Particle { pos: PhysVec {x:0., y:0.},
                                       vel: PhysVec {x:0., y:0.},
                                       mass: central_mass };
    for p in particles.iter() {
        let mut forcev = PhysVec {x : 0., y: 0.};
        for q in particles.iter() {
            if q != p {
                forcev.add(&force(p, q))
            }
        }
        forcev.add(&force(p, &dummy_central_pcl));
        let theta = p.pos.angle();
        let speed = (forcev.modulus()*p.pos.modulus()/p.mass).sqrt();
        if theta.is_nan() {
            let v = PhysVec {x: 0., y: 0.};
            vels.push(v);
        } else {
            let v = PhysVec {x: speed*theta.sin(), y: -speed*theta.cos()};
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
    return Particle {pos: PhysVec {x: rx/m, y: ry/m},
                     vel: PhysVec {x: px/m, y: py/m},
                     mass: m}
}

fn stepvel(force: PhysVec, p: &mut Particle, sense:bool) {
    if sense {
        p.vel.x += force.x/p.mass*DT;
        p.vel.y += force.y/p.mass*DT;
    } else {
        p.vel.x -= force.x/p.mass*DT;
        p.vel.y -= force.y/p.mass*DT;
    }
}

pub fn stepsim(particles: &mut Vec<Particle>, lenp: uint) {
    for i in range(0, lenp) {
        for j in range(i+1, lenp) {
            if i != j {
                let f = force(particles.get(i), particles.get(j));
                stepvel(f, particles.get_mut(i), true);
                stepvel(f, particles.get_mut(j), false);
            }
        }
    }
    for p in particles.mut_iter() {
        p.steppos();
    }
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
