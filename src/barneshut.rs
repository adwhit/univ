use physics::{Particle, PhysVec, force};

struct Branch<'a> {
    tl : Box<Node<'a>>,
    tr : Box<Node<'a>>,
    bl : Box<Node<'a>>,
    br : Box<Node<'a>>
}

impl<'a> Branch<'a> {
    fn new() -> Branch<'a> {
        Branch  { tl: box Zero,
               tr: box Zero,
               bl: box Zero,
               br: box Zero }
    }
}

enum Node<'a> {
    Many(BoxStats, Branch<'a>),
    One(&'a Particle),
    Zero
}

struct BoxStats {
    pos: PhysVec,
    com: Particle,
    width: f64,
    height: f64,
    num_particles: uint
}

pub struct QuadTree<'a> {
    pub root: Node<'a>
}

impl<'a> QuadTree<'a> {
    pub fn new<'a> (particles: Vec<&'a Particle>) -> QuadTree<'a> {
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

    fn force(&self, p: &Particle) -> Option<PhysVec> {
        bh_force(p, &self.root, 0.5)
    }
}

pub fn find_bounding_box(particles: &Vec<&Particle>) -> (f64, f64, f64, f64) {
    let mut xmax = Float::neg_infinity(); let mut ymax = Float::neg_infinity();
    let mut xmin = Float::infinity();     let mut ymin = Float::infinity();
    for p in particles.iter() {
        if p.pos.x > xmax { xmax = p.pos.x }
        if p.pos.y > ymax { ymax = p.pos.y }
        if p.pos.x < xmin { xmin = p.pos.x }
        if p.pos.y < ymin { ymin = p.pos.y }
    }
    (xmax, xmin, ymax, ymin)
}

fn make_branch<'a>(particles: Vec<&'a Particle>, x: f64, y: f64, xvar: f64, yvar: f64) -> Branch<'a> {
    let (tla, tra, bla, bra) = partition(particles, x, y);
    Branch { tl: box make_node(tla, x-xvar/2., y+yvar/2., xvar/2., yvar/2.),
           tr: box make_node(tra, x+xvar/2., y+yvar/2., xvar/2., yvar/2.),
           bl: box make_node(bla, x-xvar/2., y-yvar/2., xvar/2., yvar/2.),
           br: box make_node(bra, x-xvar/2., y+yvar/2., xvar/2., yvar/2.),
    }
}

fn make_node<'a>(particles: Vec<&'a Particle>, x: f64, y: f64, xvar: f64, yvar: f64) -> Node<'a> {
    let n = particles.len();
    if n > 1 { 
        let stats = calc_stats(&particles, x, y, xvar, yvar);
        return Many(stats, make_branch(particles, x, y, xvar, yvar));
    } else if n == 1 {
        return One(*particles.get(0));
    } else {
        return Zero;
    }
}

fn partition<'a>(particles: Vec<&'a Particle>, xsplit: f64, ysplit: f64) ->
    (Vec<&'a Particle>, Vec<&'a Particle>, Vec<&'a Particle>, Vec<&'a Particle>) {
    let mut tr: Vec<&Particle> = Vec::new();
    let mut br: Vec<&Particle> = Vec::new();
    let mut tl: Vec<&Particle> = Vec::new();
    let mut bl: Vec<&Particle> = Vec::new();
    for &p in particles.iter() {
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

fn calc_stats(particles: &Vec<&Particle>, x: f64, y:f64, xvar:f64, yvar:f64) -> BoxStats {
    let mut xmass_sum = 0.;
    let mut ymass_sum = 0.;
    let mut mass = 0.;
    let mut num_pcls = 0;
    for p in particles.iter() {
        xmass_sum += p.pos.x*p.mass;
        ymass_sum += p.pos.y*p.mass;
        mass += p.mass;
        num_pcls += 1;
    }
    //println!("x:{} y: {} n: {}", x, y, num_pcls);
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


pub fn bh_force(p: &Particle, node: &Node, threshold: f64) -> Option<PhysVec> {
    match *node {
        One(p2) => if p == p2 { return None } else { return Some(force(p, p2)) } ,
        Zero    => return None,
        Many(stats,ref branch) => {

            if p.pos.diff(stats.com.pos).modulus()/stats.width > threshold {
                return Some(force(p, &stats.com))
            } else {
                return Some(force_branch(p, branch, threshold))
            }
        }
    }
}

fn force_branch(p: &Particle, branch: &Branch, threshold: f64) -> PhysVec {
    let mut tot_force = PhysVec { x: 0., y: 0. };
    match bh_force(p, branch.tl, threshold) {
        Some(v) => tot_force.add(&v),
        None => ()
    }
    match bh_force(p, branch.tr, threshold) {
        Some(v) => tot_force.add(&v),
        None => ()
    }
    match bh_force(p, branch.bl, threshold) {
        Some(v) => tot_force.add(&v),
        None => ()
    }
    match bh_force(p, branch.br, threshold) {
        Some(v) => tot_force.add(&v),
        None => ()
    }
    tot_force
}


