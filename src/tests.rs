#[cfg(test)]

use barneshut::{QuadTree, find_bounding_box, bh_force};
use physics::{Particle, PhysVec};

mod barneshut;
mod physics;

fn dummy_particles(n: int) -> Vec<Particle> {
    let mut v : Vec<Particle> = Vec::new();
    for x in range(0,n) {
        v.push( Particle { pos: PhysVec { x: x as f64, y: x as f64 + 1.5 },
                           vel: PhysVec { x: x as f64, y: x as f64 },
                           mass: 1.0
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
    let v = dummy_particles(100);
    let vp = dummy_pointers(&v);
    let (xmax, xmin, ymax, ymin) = find_bounding_box(&vp);
    assert!(xmax == 99.0)
    assert!(ymax == 100.5)
    assert!(xmin == 0.)
    assert!(ymin == 1.5)
}

#[test]
fn test_tree() {
    let v = dummy_particles(100);
    let vp = dummy_pointers(&v);
    let qt = QuadTree::new(vp);
    for (ix, p) in v.iter().enumerate() {
        let f = bh_force(p, &qt.root, 0.5).unwrap();
        println!("ix: {:u}, fx: {:f}, fy: {:f}", ix, f.x, f.y);
    }
}

#[test]
fn test_bh_good_approx() {

}
