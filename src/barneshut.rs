use physics::{Particle, PhysVec, force, stepvel};
use std::{fmt, f64};
use std::sync::{Arc, mpsc};
use std::thread;
use deque;

pub static mut THRESH : f64 = 1.0;

struct Branch {
    tl : Box<Node>,
    tr : Box<Node>,
    bl : Box<Node>,
    br : Box<Node>
}

pub enum Node {
    Many(BoxStats, Branch),
    One(Particle),
    Zero
}

struct BoxStats {
    pos: PhysVec,
    com: Particle,
    width: f64,
    height: f64,
    num_particles: u32
}

pub struct QuadTree {
    root: Node,
}

impl QuadTree {
    pub fn new (particles: Vec<Particle>) -> QuadTree {
        let (xmax, xmin, ymax, ymin) = find_bounding_box(&particles);
        let x = xmax + xmin / 2.0;
        let y = ymax + ymin / 2.0;
        let mut xvar = xmax - xmin / 2.0;
        let mut yvar = ymax - ymin / 2.0;
        // set to square
        if xvar > yvar {
            yvar = xvar
        } else {
            xvar = yvar
        }
        QuadTree { root: make_node(particles, x, y, xvar, yvar) }
    }

    pub fn force(&self, p: Particle) -> PhysVec {
        bh_force(&p, &self.root).unwrap()
    }
}

pub fn find_bounding_box(particles: &Vec<Particle>) -> (f64, f64, f64, f64) {
    let mut xmax: f64 = f64::NEG_INFINITY; let mut ymax = f64::NEG_INFINITY;
    let mut xmin = f64::INFINITY;     let mut ymin = f64::INFINITY;
    for p in particles {
        if p.pos.x > xmax { xmax = p.pos.x }
        if p.pos.y > ymax { ymax = p.pos.y }
        if p.pos.x < xmin { xmin = p.pos.x }
        if p.pos.y < ymin { ymin = p.pos.y }
    }
    (xmax, xmin, ymax, ymin)
}

fn make_branch(particles: Vec<Particle>, x: f64, y: f64, xvar: f64, yvar: f64) -> Branch {
    let (tla, tra, bla, bra) = partition(&particles, x, y);
    Branch { 
           tl: Box::new(make_node(tla, x-xvar/2., y+yvar/2., xvar/2., yvar/2.)),
           tr: Box::new(make_node(tra, x+xvar/2., y+yvar/2., xvar/2., yvar/2.)),
           bl: Box::new(make_node(bla, x-xvar/2., y-yvar/2., xvar/2., yvar/2.)),
           br: Box::new(make_node(bra, x+xvar/2., y-yvar/2., xvar/2., yvar/2.)),
    }
}

fn make_node(particles: Vec<Particle>, x: f64, y: f64, xvar: f64, yvar: f64) -> Node {
    let n = particles.len();
    if n > 1 { 
        let stats = calc_stats(&particles, x, y, xvar, yvar);
        return Node::Many(stats, make_branch(particles, x, y, xvar, yvar));
    } else if n == 1 {
        return Node::One(particles[0].clone());
    } else {
        return Node::Zero;
    }
}

fn partition<'a>(particles: &Vec<Particle>, xsplit: f64, ysplit: f64) ->
    (Vec<Particle>, Vec<Particle>, Vec<Particle>, Vec<Particle>) {
    let mut tr: Vec<Particle> = Vec::new();
    let mut br: Vec<Particle> = Vec::new();
    let mut tl: Vec<Particle> = Vec::new();
    let mut bl: Vec<Particle> = Vec::new();
    for &p in particles {
        if p.pos.x > xsplit {
            if p.pos.y > ysplit {
                tr.push(p)
            } else {
                br.push(p)
            }
        } else {
            if p.pos.y > ysplit {
                tl.push(p)
            } else {
                bl.push(p)
            }
        }
    }
    (tl, tr, bl, br)
}

fn calc_stats(particles: &Vec<Particle>, x: f64, y:f64, xvar:f64, yvar:f64) -> BoxStats {
    let mut xmass_sum = 0.;
    let mut ymass_sum = 0.;
    let mut mass = 0.;
    let mut num_pcls = 0;
    for p in particles {
        xmass_sum += p.pos.x*p.mass;
        ymass_sum += p.pos.y*p.mass;
        mass += p.mass;
        num_pcls += 1;
    }
    BoxStats { pos: PhysVec { x: x, y: y },
               com: Particle { 
                   pos: PhysVec {x: xmass_sum/mass, y: ymass_sum/mass},
                   vel: PhysVec {x: 0., y: 0.},
                   mass: mass },
               width: xvar * 2.0,
               height: yvar * 2.0,
               num_particles: num_pcls
    }
}


pub fn bh_force(p: &Particle, node: &Node) -> Option<PhysVec> {
    match *node {
        Node::One(p2) => if *p == p2 { return None } else { return Some(force(p, &p2)) } ,
        Node::Zero    => return None,
        Node::Many(ref stats,ref branch) => {
            if unsafe { p.pos.diff(stats.com.pos).modulus()/stats.width > THRESH } {
                return Some(force(p, &stats.com))
            } else {
                return Some(force_branch(p, branch))
            }
        }
    }
}

fn force_branch(p: &Particle, branch: &Branch) -> PhysVec {
    let mut tot_force = PhysVec { x: 0., y: 0. };
    match bh_force(p, &branch.tl) {
        Some(v) => tot_force.add(&v),
        None => ()
    }
    match bh_force(p, &branch.tr) {
        Some(v) => tot_force.add(&v),
        None => ()
    }
    match bh_force(p, &branch.bl) {
        Some(v) => tot_force.add(&v),
        None => ()
    }
    match bh_force(p, &branch.br) {
        Some(v) => tot_force.add(&v),
        None => ()
    }
    tot_force
}

pub fn pcl_pointers<'a>(particles: &'a Vec<Particle>) -> Vec<&'a Particle> {
    let mut v : Vec<&Particle> = Vec::with_capacity(particles.len());
    for p in particles {
        v.push(p)
    }
    v
}


pub fn stepsim(particles: &mut Vec<Particle>) {
    let mut frcs : Vec<PhysVec> =  Vec::with_capacity(particles.len());
    {
        let qt = QuadTree::new(particles.clone());
        for &p in particles.iter() {
            frcs.push(qt.force(p));
        }
    }
    for (mut p, &f) in particles.iter_mut().zip(frcs.iter()) {
        stepvel(p, f, true);
        p.steppos();
    }
}

pub fn stepsim_par(particles: &mut Vec<Particle>) {
    let lenp = particles.len();
    let rcqt = Arc::new(QuadTree::new(particles.clone()));

    let (tx, rx) = mpsc::channel();              //channel to receive results
    let pool = deque::BufferPool::new();   //work pool
    let (worker, stealer) = pool.deque();

    for (ix, &p) in particles.iter().enumerate() {
        worker.push((ix as u32, p.clone()))             //construct queue
    }

    for _ in 0..7 {
        let localqt = rcqt.clone();
        let localtx = tx.clone();
        let stlr = stealer.clone();
        thread::spawn(move || { steal_work(localqt, &localtx, stlr) });
    }
    for _ in 0..lenp {
        let (ix, pv) = rx.recv().unwrap();
        let p = &mut particles[ix as usize];
        stepvel(p, pv, true);
        p.steppos();
    }
}

fn steal_work(qt: Arc<QuadTree>, tx: &mpsc::Sender<(u32, PhysVec)>, stealer: deque::Stealer<(u32, Particle)>) {
    loop {
        match stealer.steal() {
            deque::Stolen::Empty => break,
            deque::Stolen::Abort => continue,
            deque::Stolen::Data((ix,p)) => {tx.send((ix,qt.force(p))); ()}
        }
    }
}

impl fmt::Display for BoxStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X:{} Y:{} Width:{} Height:{} Mass:{} NumPcls: {}",
        self.pos.x, self.pos.y, self.width, self.height, self.com.mass, self.num_particles)
    }
}
