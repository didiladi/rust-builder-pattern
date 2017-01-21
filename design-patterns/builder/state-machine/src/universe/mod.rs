
use std::f64;
use std::convert::From;

const GRAVITATIONAL_CONSTANT: f64 = 0.00000000006674; // N*m^2/kg^2
const SPEED_OF_LIGHT: u64 = 299_792_458; // m/s

const INITIAL_NAME: &'static str = "Unknown";
const DEFAULT_DISCOVERED_YEAR: u16 = 2017;

#[derive(Clone)]
pub enum Type {
    SuperMassive,
    IntermediateMassive,
    Stellar,
    Micro
}

pub struct BlackHole {

    name: String,
    discovered_by: String,
    year_of_discovery: u16,

    mass: Option<f64>,
    angular_momentum: Option<f64>,
    electric_charge: Option<f64>,

    classification: Option<Type>
}

pub trait State {}


pub struct NameBuilder;
impl State for NameBuilder {}

pub struct DiscoveredByBuilder;
impl State for DiscoveredByBuilder {}

impl From<NameBuilder> for DiscoveredByBuilder {
    fn from(_: NameBuilder) -> DiscoveredByBuilder {
        DiscoveredByBuilder
    }
}

pub struct YearOfDiscoveryBuilder;
impl State for YearOfDiscoveryBuilder {}

impl From<DiscoveredByBuilder> for YearOfDiscoveryBuilder {
    fn from(_: DiscoveredByBuilder) -> YearOfDiscoveryBuilder { 
        YearOfDiscoveryBuilder
    }
}

pub struct OptionalParamsBuilder;
impl State for OptionalParamsBuilder {}
 
impl From<YearOfDiscoveryBuilder> for OptionalParamsBuilder {
    fn from(_: YearOfDiscoveryBuilder) -> OptionalParamsBuilder { 
        OptionalParamsBuilder
    }
}

pub struct BlackHoleBuilder<S: State> {
    black_hole: BlackHole,
    #[allow(dead_code)]
    state: S
}

impl<T: State> BlackHoleBuilder<T> {
    fn transition<X: State + From<T>>(self, state: X) -> BlackHoleBuilder<X> {
        BlackHoleBuilder {
            black_hole: self.black_hole,
            state: state
        }
    }
}

impl BlackHoleBuilder<NameBuilder> {
 
    fn new() -> Self {

        BlackHoleBuilder {
            black_hole : BlackHole {
                name:               INITIAL_NAME.to_string(),
                discovered_by:      INITIAL_NAME.to_string(),
                year_of_discovery:  DEFAULT_DISCOVERED_YEAR,
                mass:               None,
                angular_momentum:   None,
                electric_charge:    None,
                classification:     None
            },
            state : NameBuilder
        }
    }

    pub fn name(self, name: String) -> BlackHoleBuilder<DiscoveredByBuilder> {
        self.transition(DiscoveredByBuilder)
    }
}

impl BlackHoleBuilder<DiscoveredByBuilder> {
    pub fn discovered_by(self, discovered_by: String) -> BlackHoleBuilder<YearOfDiscoveryBuilder> {
        self.transition(YearOfDiscoveryBuilder)
    }
}

impl BlackHoleBuilder<YearOfDiscoveryBuilder> {
    pub fn year_of_discovery(self, year_of_discovery: u16) -> BlackHoleBuilder<OptionalParamsBuilder> {
        self.transition(OptionalParamsBuilder)
    }
}

impl BlackHoleBuilder<OptionalParamsBuilder> {

    pub fn mass(self, mass: f64) -> Self {
        self
    }

    pub fn angular_momentum(self, angular_momentum: f64) -> Self {
        self
    }

    pub fn electric_charge(self, electric_charge: f64) -> Self {
        self
    }

    pub fn classification(self, classification: Type) -> Self {
        self
    }

    /// Builds the black hole
    /// All members of BlackHoleBuilder are now moved to BlackHole
    ///
    ///
    pub fn build(self) -> BlackHole {
        self.black_hole
    }

    /*fn build_copy(&self) -> BlackHole {
        BlackHole {

            name:               self.black_hole.name,
            discovered_by:      self.black_hole.discovered_by,
            year_of_discovery:  self.black_hole.year_of_discovery,

            mass:               self.black_hole.mass,
            angular_momentum:   self.black_hole.angular_momentum,
            electric_charge:    self.black_hole.electric_charge,

            classification:     self.black_hole.classification.clone()
        }
    }
    */
}

impl BlackHole {

    pub fn new() -> BlackHoleBuilder<NameBuilder> {
        BlackHoleBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Calculate the radius of the event horizon, commonly referred as 'Schwarzschild radius'
    /// See: https://en.wikipedia.org/wiki/Schwarzschild_radius
    ///
    ///      2 * G * M
    /// rs = ---------
    ///        c ^ 2
    ///
    pub fn calc_event_horizon_radius(&self) -> Option<f64> {
        // TODO check the calculation
        self.mass.map(|mass| (2.0 * GRAVITATIONAL_CONSTANT * mass) / (SPEED_OF_LIGHT.pow(2) as f64))
    }
}
