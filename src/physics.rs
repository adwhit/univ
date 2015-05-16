use std::{f64, fmt};
use config::{GalaxyCfg, GalaxyShape};
use config;
use rand;

pub static mut DT: f64 = 0.05;
static EPS: f64 = 0.;

#[derive(PartialEq, Clone, Copy)]
pub struct Particle {
    pub pos : PhysVec,
    pub vel : PhysVec,
    pub mass: f64
}

#[derive(PartialEq, Copy, Clone)]
pub struct PhysVec {
    pub x : f64,
    pub y : f64
}

impl PhysVec {
    pub fn add(&mut self, other: &PhysVec) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn dot(&self, other: &PhysVec) -> f64 {
    self.x * other.x + self.y * other.y
    }

    pub fn modulus(&self) -> f64 {
        (self.x * self.x + self.y* self.y).sqrt()
    }

    //vector pointing from v1 towards v2
    pub fn diff(&self, v2: PhysVec) -> PhysVec {
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

    pub fn steppos(&mut self) {
        unsafe {
            self.pos.x = self.pos.x + self.vel.x*DT;
            self.pos.y = self.pos.y + self.vel.y*DT;
        }
    }

}


//force is calculated as pointing from particle 1 towards particle 2
pub fn force(p1: &Particle, p2: &Particle) -> PhysVec {
    let disp = p1.pos.diff(p2.pos);
    let dist = disp.modulus() + EPS;
    let f = p1.mass * p2.mass / dist; // force magnitude
    PhysVec { x: f*disp.x/dist, y: f*disp.y/dist }
}


fn spawn_circular_galaxy(max_radius: f64, nrings: u32, num_bodys: u32) -> Vec<Particle> {
    //create galaxy with concentric equally-spaced rings. The number of bodys in each ring is proportional
    //to the square of the radius of the ring
    let mut particles: Vec<Particle> = Vec::new();
    let pi =  f64::consts::PI;
    let nsqr: Vec<u32> = (0..nrings).map(|ix| (ix+1)*(ix+1)).collect();
    let sum_sqr = nsqr.iter().sum::<u32>() as f64 ;
    let nbody: Vec<i32> = nsqr.iter().map(|x| ((x*num_bodys) as f64 /sum_sqr) as i32).collect();
    let radii: Vec<f64> = (0..nrings).map(|ix| max_radius/(nrings as f64)*(ix as f64 + 1.0)).collect();
    for (&n, &r) in nbody.iter().zip(radii.iter()) {
        for i in 1..(n+1) {
            let theta = (i as f64)/(n as f64)*2.0*pi;
            particles.push(Particle {pos:PhysVec {x: r*theta.cos(), y: r*theta.sin() }, 
                                     vel:PhysVec {x: 0., y: 0.},
                                     mass:1. });
        }
    }
    particles
}

fn spawn_random_galaxy_weighted(radius: f64, num_bodys: u32) -> Vec<Particle> {
    let pi =  f64::consts::PI;
    let mut particles: Vec<Particle> = Vec::new();
    for _ in 0..num_bodys {
        let theta = rand::random::<f64>()*2.0*pi;
        let r = rand::random::<f64>()*radius;
        let x =  r*theta.cos();
        let y =  r*theta.sin();
        particles.push(Particle {pos:PhysVec {x: x,  y: y },
                                 vel:PhysVec {x: 0., y: 0.},
                                 mass:1. });
    }
    particles
}

fn spawn_random_galaxy_even(radius: f64, num_bodys: u32) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    let r2 = radius * radius;
    let mut ct = 0;
    while ct < num_bodys {
        let x = (rand::random::<f64>() - 0.5)*2.0*radius;
        let y = (rand::random::<f64>() - 0.5)*2.0*radius;
        if x*x + y*y < r2 {
            particles.push(Particle {pos:PhysVec {x: x,  y: y },
                                     vel:PhysVec {x: 0., y: 0.},
                                     mass:1. });
        }
        ct +=1;
    }
    particles
}

fn galilean_offset(particles: &mut Vec<Particle>, central_pcl: &Particle) {
    //offset all particles by given position velocity
    for mut p in particles {
        p.pos.add(&central_pcl.pos);
        p.vel.add(&central_pcl.vel);
    }
}

pub fn make_galaxy(gal: GalaxyCfg) -> Vec<Particle> {
    let central_pcl = Particle { 
        pos: PhysVec { x: gal.posx.unwrap(), y: gal.posy.unwrap() },
        vel: PhysVec { x: gal.velx.unwrap(), y: gal.vely.unwrap() },
        mass: gal.central_mass.unwrap()
    };
    let mut particles = match gal.shape.unwrap() {
        config::GalaxyShape::RandomWeighted => spawn_random_galaxy_weighted(gal.radius.unwrap(), gal.nbody),
        config::GalaxyShape::RandomEven => spawn_random_galaxy_even(gal.radius.unwrap(), gal.nbody),
        config::GalaxyShape::Concentric(nrings) => spawn_circular_galaxy(gal.radius.unwrap(), nrings, gal.nbody)
    };

    match gal.kinetics.unwrap() {
        config::GalaxyKinetics::ZeroVel               => (),
        config::GalaxyKinetics::RandomVel(minv, maxv) => init_random_vel(&mut particles, minv, maxv),
        config::GalaxyKinetics::CircularOrbit         => init_circular_orbits(&mut particles, central_pcl.mass)
    };
    galilean_offset(&mut particles, &central_pcl);
    particles.push(central_pcl);
    particles
}

fn init_random_vel(particles: &mut Vec<Particle>, minv: f64, maxv: f64) {
    for p in particles {
        p.vel.x = rand::random::<f64>()*(maxv - minv) + minv;
        p.vel.y = rand::random::<f64>()*(maxv - minv) + minv;
        if rand::random::<bool>() { p.vel.x *= -1.0 };
        if rand::random::<bool>() { p.vel.y *= -1.0 };
    }
    
}

fn init_circular_orbits(particles: &mut Vec<Particle>, central_mass: f64) {
    //Calculate force and velocities to create a circular orbit
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
    for (p, &v) in particles.iter_mut().zip(vels.iter()) {
        p.vel  = v;
    }
}

pub fn stepvel(p: &mut Particle, force: PhysVec, sense:bool) {
    unsafe {
        if sense {
            p.vel.x += force.x/p.mass*DT;
            p.vel.y += force.y/p.mass*DT;
        } else {
            p.vel.x -= force.x/p.mass*DT;
            p.vel.y -= force.y/p.mass*DT;
        }
    }
}

pub fn stepsim(particles: &mut Vec<Particle>) {
    let lenp = particles.len();
    for i in 0..lenp {
        for j in i+1..lenp {
            if i != j {
                let f = force(&particles[i], &particles[j]);
                stepvel(&mut particles[i], f, true);
                stepvel(&mut particles[j], f, false);
            }
        }
    }
    for p in particles {
        p.steppos();
    }
}

fn total_ke(particles: &Vec<Particle>) -> f64 {
    let mut ke = 0.;
    for p in particles {
        ke += p.kinetic_energy()
    }
    ke
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PX:{:0.2}\tPY:{:0.2}\tVX:{:0.2}\tVY:{:0.2}\tMass:{:0.2}\tKE:{:0.2}",
            self.pos.x, self.pos.y, self.vel.x, self.vel.y, self.mass, 
            self.kinetic_energy())
    }
}
