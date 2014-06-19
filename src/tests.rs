#[cfg(test)]

use barneshut::{QuadTree, find_bounding_box, bh_force, pcl_pointers, bh_stepsim};
use physics::{Particle, PhysVec, force};

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

#[test]
fn test_bounding_box() {
    let v = dummy_particles(100);
    let vp = pcl_pointers(&v);
    let (xmax, xmin, ymax, ymin) = find_bounding_box(&vp);
    assert!(xmax == 99.0)
    assert!(ymax == 100.5)
    assert!(xmin == 0.)
    assert!(ymin == 1.5)
}

#[test]
fn test_tree() {
    let threshold = 1.0;
    let pcls = dummy_particles(10);
    let pptrs = pcl_pointers(&pcls);
    let qt = QuadTree::new(pptrs, threshold);
    for (ix, p) in pcls.iter().enumerate() {
        let bhfrc = qt.force(p);
        let mut frc = PhysVec {x: 0., y: 0.};
        for q in pcls.iter() {
            if p != q {
                frc.add(&force(p, q))
            }
        }
        println!("ix: {:u} diff: {:0.2f}%", ix, (frc.x - bhfrc.x)/frc.x*100.);
    }
}

#[test]
fn test_qt() {
    let threshold = 2.0;
    let mut pcls = dummy_particles(100);
    let l = pcls.len();
    bh_stepsim(&mut pcls, l, threshold)
}
