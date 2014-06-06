use physics::Particle;

enum Node {
    Many(Stats,Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    One(Stats),
    Zero
}

struct Stats {
    x : f64,
    y : f64,
    centre_of_mass: f64,
    mass : f64
}

fn find_bounding_box(particles: Vec<Particle>) -> (f64, f64, f64, f64) {
    let mut xmax = Float::neg_infinity(); let mut ymax = Float::neg_infinity();
    let mut xmin = Float::infinity();     let mut ymin = Float::infinity();
    for p in particles.iter() {
        if p.pos.x > xmax { xmax = p.pos.x }
        if p.pos.y > ymax { ymax = p.pos.y }
        if p.pos.x < xmin { xmin = p.pos.x }
        if p.pos.y < ymin { ymin = p.pos.y }
    }
}


