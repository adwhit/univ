extern crate sdl2;

use std::io::timer::sleep;

static WIDTH: int = 1024;
static HEIGHT: int = 768;
static COLDEPTH: int = 32;
static FPS: int = 60;
static DT: f64 = 1.0;
static EPS: f64 = 0.001;
static BPP: int = 4; 

struct Particle {
    pos : Vector,
    vel : Vector,
    mass: f64
}

struct Vector {
    x : f64,
    y : f64
}


fn force(p1: &Particle, p2: &Particle) -> Vector {
    let disp = diff(p1.pos, p2.pos);
    let dist = modls(disp);
    let f = p1.mass * p2.mass / (dist + EPS); // force magnitude
    Vector { x: f*dist/disp.x, y: f*dist/disp.y }
}

fn step(m: &mut Particle, M: &mut Particle) {
    // assume M doesn't change
    // update positions
    m.pos.x = m.pos.x + m.vel.x;
    m.pos.y = m.pos.y + m.vel.y;
    // update velocities
    let f = force(m, M);
    m.vel.x += f.x/m.mass*DT;
    m.vel.y += f.y/m.mass*DT;
}

fn dot(v1: Vector, v2: Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}

fn modls(v : Vector) -> f64 {
    (v.x * v.x + v.y* v.y).sqrt()
}

fn diff(v1 : Vector, v2: Vector) -> Vector {
    Vector { x: v1.x - v2.x, y: v1.y -v2.y }
}

/*
fn fill_circle(screen: &Surface, x: f64, y: f64, r:f64, col:RGB) {
    let mut dy = 0f64;
    while dy < r {
        let dx: f64 = (2 * r * dy - dy * dy).sqrt().floor();
        let mut xi: int = (x - dx) as int;
        // set pixel cy + r -dy * pitch * xi * BPP
        target_pixel_a = screen.pixels + ((int)(cy + r - dy)) * screen->pitch + x * BPP;
        Uint8 *target_pixel_b = (Uint8 *)surface->pixels + ((int)(cy - r + dy)) * surface->pitch + x * BPP;
        while x <= cx + dx {
            *(Uint32 *)target_pixel_a = pixel;
            *(Uint32 *)target_pixel_b = pixel;
            target_pixel_a += BPP;
            target_pixel_b += BPP;
            x += 1;
        }

    }
}
*/

fn main() {
    sdl2::init(sdl2::InitVideo);

    let window = match sdl2::video::Window::new("Univ", sdl2::video::PosCentered, sdl2::video::PosCentered, WIDTH, HEIGHT, sdl2::video::OpenGL) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    for x in range(0u8, 255) {
        renderer.set_draw_color(sdl2::pixels::RGB(x, 255-x, 0));
        renderer.clear();
        renderer.present();
        sleep(10);
    }

    sdl2::quit();

    //let mut m1 = Particle { pos:Vector {x: 1, y: 1}, vel:Vector {x:0,y:0}, mass:1 };
    //let mut m2 = Particle { pos:Vector {x:-1, y:-1}, vel:Vector {x:0,y:0}, mass:1 };

}

