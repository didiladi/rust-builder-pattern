
use std::f64;

const GRAVITATIONAL_CONSTANT: f64 = 0.00000000006674; // N*m^2/kg^2
const SPEED_OF_LIGHT: u64 = 299_792_458; // m/s

const DEFAULT_DISCOVERED_BY: &'static str = "Unknown";
const DEFAULT_DISCOVERED_YEAR: u16 = 2017;

#[derive(Clone, Debug)]
pub enum Type {
    SuperMassive,
    IntermediateMassive,
    Stellar,
    Micro
}

pub struct BlackHole {

    pub name: String,
    pub discovered_by: String,
    pub year_of_discovery: u16,

    pub mass: Option<f64>,
    pub angular_momentum: Option<f64>,
    pub electric_charge: Option<f64>,

    pub classification: Option<Type>
}

pub struct BlackHoleBuilder {

    name: String,
    discovered_by: Option<String>,
    year_of_discovery: Option<u16>,

    mass: Option<f64>,
    angular_momentum: Option<f64>,
    electric_charge: Option<f64>,

    classification: Option<Type>
}

impl BlackHoleBuilder {

    pub fn new(name: &str) -> BlackHoleBuilder {
        BlackHoleBuilder {
            name:               name.to_string(),
            discovered_by:      None,
            year_of_discovery:  None,
            mass:               None,
            angular_momentum:   None,
            electric_charge:    None,
            classification:     None
        }
    }

    pub fn discovered_by<I>(mut self, discovered_by: I) -> BlackHoleBuilder
        where I: Into<Option<String>> {

        self.discovered_by = discovered_by.into();
        self
    }

    pub fn year_of_discovery<I>(mut self, year_of_discovery: I) -> BlackHoleBuilder
        where I: Into<Option<u16>> {

        self.year_of_discovery = year_of_discovery.into();
        self
    }

    pub fn mass<I>(mut self, mass: I) -> BlackHoleBuilder
        where I: Into<Option<f64>> {

        self.mass = mass.into();
        self
    }

    pub fn angular_momentum<I>(mut self, angular_momentum: I) -> BlackHoleBuilder
        where I: Into<Option<f64>> {

        self.angular_momentum = angular_momentum.into();
        self
    }

    pub fn electric_charge<I>(mut self, electric_charge: I) -> BlackHoleBuilder
        where I: Into<Option<f64>> {

        self.electric_charge = electric_charge.into();
        self
    }

    pub fn classification<I>(mut self, classification: I) -> BlackHoleBuilder
        where I: Into<Option<Type>> {

        self.classification = classification.into();
        self
    }

    /// Builds the black hole
    /// All members of BlackHoleBuilder are now moved to BlackHole
    ///
    ///
    pub fn build(self) -> BlackHole {
        BlackHole {

            name:               self.name,
            discovered_by:      self.discovered_by.unwrap_or(DEFAULT_DISCOVERED_BY.to_string()),
            year_of_discovery:  self.year_of_discovery.unwrap_or(DEFAULT_DISCOVERED_YEAR),

            mass:               self.mass,
            angular_momentum:   self.angular_momentum,
            electric_charge:    self.electric_charge,

            classification:     self.classification
        }
    }

    pub fn build_copy(&self) -> BlackHole {
        BlackHole {

            name:               self.name.clone(),
            discovered_by:      self.discovered_by.clone().map_or_else(
                                    | | DEFAULT_DISCOVERED_BY.to_string(),
                                    |value| value.clone()),
            year_of_discovery:  self.year_of_discovery.unwrap_or(DEFAULT_DISCOVERED_YEAR),

            mass:               self.mass,
            angular_momentum:   self.angular_momentum,
            electric_charge:    self.electric_charge,

            classification:     self.classification.clone()
        }
    }
}

impl BlackHole {

    /// Calculate the radius of the event horizon, commonly referred as 'Schwarzschild radius'
    /// See: https://en.wikipedia.org/wiki/Schwarzschild_radius
    ///
    ///      2 * G * M
    /// rs = ---------
    ///        c ^ 2
    ///
    pub fn calc_event_horizon_radius(&self) -> Option<f64> {
        self.mass.map(|mass| (2.0 * GRAVITATIONAL_CONSTANT * mass) / (SPEED_OF_LIGHT.pow(2) as f64))
    }
}

