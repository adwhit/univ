use physics::Particle;

struct Node {
    tl : Box<Child<Particle>>,
    tr : Box<Child<Particle>>,
    bl : Box<Child<Particle>>,
    br : Box<Child<Particle>>
}

impl Node {
    fn new() -> Node {
        Node { tl: box Zero,
               tr: box Zero,
               bl: box Zero,
               br: box Zero }
    }
}

enum Child<Particle> {
    Many(BoxAttr,Box<Node>),
    One(Particle),
    Zero
}

struct BoxAttr {
    x : f64,
    y : f64,
    num_particles: uint,
    centre_of_mass: f64,
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
    QuadTree { root: root }
}
