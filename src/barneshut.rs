use physics::Particle;

struct Node<'a> {
    tl : Box<Child<'a>>,
    tr : Box<Child<'a>>,
    bl : Box<Child<'a>>,
    br : Box<Child<'a>>
}

impl<'a> Node<'a> {
    fn new() -> Node<'a> {
        Node  { tl: box Zero,
               tr: box Zero,
               bl: box Zero,
               br: box Zero }
    }
}

enum Child<'a> {
    Many(BoxStats, Node<'a>),
    One(&'a Particle),
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

struct QuadTree<'a> {
    root: Node<'a>
}

fn find_bounding_box(particles: &Vec<&Particle>) -> (f64, f64, f64, f64) {
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

fn create_tree(particles: Vec<&Particle>) -> QuadTree {
    let (xmax, xmin, ymax, ymin) = find_bounding_box(&particles);
    let x = xmax + xmin / 2.0;
    let y = ymax + ymin / 2.0;
    let mut root = Node::new();
    let qt = QuadTree { root: root };
    let (tl, tr, bl, br) = partition(particles, x, y);
    qt
}

fn make_node<'a>(particles: Vec<&'a Particle>, x: f64, y: f64, xvar: f64, yvar: f64) -> Node<'a> {
    let (tla, tra, bla, bra) = partition(particles, x, y);
    Node { tl: box make_child(tla, x-xvar/2., y+yvar/2., xvar/2., yvar/2.),
           tr: box make_child(tra, x+xvar/2., y+yvar/2., xvar/2., yvar/2.),
           bl: box make_child(bla, x-xvar/2., y-yvar/2., xvar/2., yvar/2.),
           br: box make_child(bra, x-xvar/2., y+yvar/2., xvar/2., yvar/2.),
    }
}

fn make_child<'a>(particles: Vec<&'a Particle>, x: f64, y: f64, xvar: f64, yvar: f64) -> Child<'a> {
    let n = particles.len();
    if n > 1 { 
        let stats = calc_stats(&particles, x, y);
        return Many(stats, make_node(particles, x, y, xvar, yvar));
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

fn calc_stats(particles: &Vec<&Particle>, x: f64, y:f64) -> BoxStats {
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
    println!("x:{} y: {} n:{}", x, y, num_pcls);
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

    fn dummy_pointers<'a>(particles: &'a Vec<Particle>) -> Vec<&'a Particle> {
        let mut v : Vec<&Particle> = Vec::new();
        for p in particles.iter() {
            v.push(p)
        }
        v
    }

    #[test]
    fn test_bounding_box() {
        let v = dummy_particles();
        let vp = dummy_pointers(&v);
        let (xmax, xmin, ymax, ymin) = find_bounding_box(&vp);
        assert!(xmax == 99.0)
        assert!(ymax == 100.0)
        assert!(xmin == 0.)
        assert!(ymin == 1.)
    }

    #[test]
    fn test_tree() {
        let v = dummy_particles();
        let vp = dummy_pointers(&v);
        create_tree(vp);
    }
}
