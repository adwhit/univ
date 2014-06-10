use physics::Particle;

struct Node {
    tl : Box<Child>,
    tr : Box<Child>,
    bl : Box<Child>,
    br : Box<Child>
}

impl Node {
    fn new() -> Node {
        Node  { tl: box Zero,
               tr: box Zero,
               bl: box Zero,
               br: box Zero }
    }
}

enum Child {
    Many(BoxStats,Box<Node>),
    One(Particle),
    Zero
}

struct BoxStats {
    x : f64,
    y : f64,
    num_particles: uint,
    x_com: f64,
    y_com: f64,
    mass : f64
}

struct QuadTree {
    root: Node
}

fn find_bounding_box(particles: &Vec<Particle>) -> (f64, f64, f64, f64) {
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

fn create_tree(particles: &Vec<Particle>) -> QuadTree {
    let (xmax, xmin, ymax, ymin) = find_bounding_box(particles);
    let xcentre = xmax + xmin / 2.0;
    let ycentre = ymax + ymin / 2.0;
    let mut root = Node::new();
    let qt = QuadTree { root: root };
    let (tl, tr, bl, br) = partition(particles);
    qt
}

fn descend(particles: &Vec<Particle>) -> Node {
    let (tla, tra, bla, bra) = partition(particles);
    let tl = if tls.len() > 1 { } //XXX Finish!
}

fn partition(particles: &Vec<Particle>, xsplit: f64, ysplit: f64) ->
    (Vec<Particle>, Vec<Particle>, Vec<Particle>, Vec<Particle>) {
    let mut tr: Vec<Particle> = Vec::new();
    let mut br: Vec<Particle> = Vec::new();
    let mut tl: Vec<Particle> = Vec::new();
    let mut bl: Vec<Particle> = Vec::new();
    for p in particles.iter() {
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

fn calc_stats(particles: &Vec<Particle>, x: f64, y:f64) -> BoxStats {
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
    BoxStats { x: x, 
               y: y, 
               num_particles: num_pcls,
               x_com: xmass_sum/mass,
               y_com: ymass_sum/mass,
               mass: mass }
}


#[cfg(test)]
mod bhtests {

    use barneshut::{find_bounding_box, create_tree};
    use physics::{Particle, PhysVec};

    fn dummy_particles() -> Vec<Particle> {
        let mut v : Vec<Particle> = Vec::new();
        for x in range(0,100) {
            v.push( Particle { pos: PhysVec { x: x as f64, y: 100. - x as f64 },
                               vel: PhysVec { x: x as f64, y: 100. - x as f64 },
                               mass: x as f64
                             } )
        }
        v
    }

    #[test]
    fn test_bounding_box() {
        let v = dummy_particles();
        let (xmax, xmin, ymax, ymin) = find_bounding_box(&v);
        assert!(xmax == 99.0)
        assert!(ymax == 100.0)
        assert!(xmin == 0.)
        assert!(ymin == 1.)
    }

    #[test]
    fn test_tree() {
        let v = dummy_particles();
        create_tree(v)
    }

}
