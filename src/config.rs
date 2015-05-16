#[derive(RustcDecodable, Debug)]
pub enum SimType {
    BarnesHut,
    BarnesHutParallel,
    Classical
}

#[derive(RustcDecodable, Debug)]
pub struct Config {
    pub display :  Display,
    pub galaxies:  Vec<GalaxyCfg>,
    pub sim:       SimType,
    pub threshold: f64,
    pub dt       : f64
}

#[derive(RustcDecodable, Debug)]
pub struct ConfigOpt {
    pub display :  Option<DisplayOpt>,
    pub galaxies  :  Vec<GalaxyCfg>,
    pub sim:       Option<SimType>,
    pub threshold: Option<f64>,
    pub dt       : Option<f64>
}

#[derive(RustcDecodable, Debug, Clone, Copy)]
pub struct Display {
    pub width: i32,
    pub height: i32
}

#[derive(RustcDecodable, Debug)]
pub struct DisplayOpt {
    pub width: Option<u32>,
    pub height: Option<u32>
}

#[derive(RustcDecodable, Debug, Clone)]
pub struct GalaxyCfg {
    pub posx: Option<f64>,
    pub posy: Option<f64>,
    pub velx: Option<f64>,
    pub vely: Option<f64>,
    pub radius: Option<f64>,
    pub nbody: u32,                        // the only mandatory field
    pub shape: Option<GalaxyShape>,
    pub kinetics: Option<GalaxyKinetics>,
    pub central_mass: Option<f64>,
    pub other_mass: Option<f64>
}

//Represents internal shape of galaxy
#[derive(RustcDecodable, Debug, Clone, Copy)]
pub enum GalaxyShape {
    RandomWeighted,
    RandomEven,
    Concentric(u32)
}

#[derive(RustcDecodable, Debug, Clone, Copy)]
pub enum GalaxyKinetics {
    RandomVel(f64, f64),
    CircularOrbit,
    ZeroVel,
}

