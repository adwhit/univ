#[deriving(Decodable, Show)]
pub enum SimType {
    BarnesHut,
    BarnesHutParallel,
    Classical
}

#[deriving(Decodable, Show)]
pub struct Config {
    pub display :  Display,
    pub galaxies:  Vec<GalaxyCfg>,
    pub sim:       SimType,
    pub threshold: f64,
    pub dt       : f64
}

#[deriving(Decodable, Show)]
pub struct ConfigOpt {
    pub display :  Option<DisplayOpt>,
    pub galaxies:  Vec<GalaxyCfg>,
    pub sim:       Option<SimType>,
    pub threshold: Option<f64>,
    pub dt       : Option<f64>
}

#[deriving(Decodable, Show)]
pub struct Display {
    pub width: uint,
    pub height: uint
}

#[deriving(Decodable, Show)]
pub struct DisplayOpt {
    pub width: Option<uint>,
    pub height: Option<uint>
}

#[deriving(Decodable, Show)]
pub struct GalaxyCfg {
    pub posx: Option<f64>,
    pub posy: Option<f64>,
    pub velx: Option<f64>,
    pub vely: Option<f64>,
    pub radius: Option<f64>,
    pub nbody: uint,                        // the only mandatory field
    pub shape: Option<GalaxyShape>,
    pub kinetics: Option<GalaxyKinetics>,
    pub central_mass: Option<f64>,
    pub other_mass: Option<f64>
}

//Represents internal shape of galaxy
#[deriving(Decodable, Show)]
pub enum GalaxyShape {
    RandomWeighted,
    RandomEven,
    Concentric(uint)
}

#[deriving(Decodable, Show)]
pub enum GalaxyKinetics {
    RandomVel(f64, f64),
    CircularOrbit,
    ZeroVel,
}

