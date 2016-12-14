
use std::f64;

const GRAVITATIONAL_CONSTANT: f64 = 0.00000000006674; // N*m^2/kg^2
const SPEED_OF_LIGHT: u64 = 299_792_458; // m/s

const DEFAULT_DISCOVERED_BY: &'static str = "Unknown";
const DEFAULT_DISCOVERED_YEAR: u16 = 2017;

enum Type {

    SuperMassive,
    IntermediateMassive,
    Stellar,
    Micro
}

struct BlackHole<'a> {

    name: &'a str,
    discovered_by: &'a str,
    year_of_discovery: u16,

    mass: Option<f64>,
    angular_momentum: Option<f64>,
    electric_charge: Option<f64>,

    classification: Option<Type>
}

struct BlackHoleBuilder<'a> {

    name: &'a str,
    discovered_by: Option<&'a str>,
    year_of_discovery: Option<u16>,

    mass: Option<f64>,
    angular_momentum: Option<f64>,
    electric_charge: Option<f64>,

    classification: Option<Type>
}

impl<'a> BlackHoleBuilder<'a> {

    fn new(name: &'a str) -> BlackHoleBuilder<'a> {
        BlackHoleBuilder {
            name:               name,
            discovered_by:      None,
            year_of_discovery:  None,
            mass:               None,
            angular_momentum:   None,
            electric_charge:    None,
            classification:     None
        }
    }

    fn discovered_by<I>(mut self, discovered_by: I) -> BlackHoleBuilder<'a>
        where I: Into<Option<&'a str>> {

        self.discovered_by = discovered_by.into();
        self
    }

    fn year_of_discovery<I>(mut self, year_of_discovery: I) -> BlackHoleBuilder<'a>
        where I: Into<Option<u16>> {

        self.year_of_discovery = year_of_discovery.into();
        self
    }

    fn mass<I>(mut self, mass: I) -> BlackHoleBuilder<'a>
        where I: Into<Option<f64>> {

        self.mass = mass.into();
        self
    }

    fn angular_momentum<I>(mut self, angular_momentum: I) -> BlackHoleBuilder<'a>
        where I: Into<Option<f64>> {

        self.angular_momentum = angular_momentum.into();
        self
    }

    fn electric_charge<I>(mut self, electric_charge: I) -> BlackHoleBuilder<'a>
        where I: Into<Option<f64>> {

        self.electric_charge = electric_charge.into();
        self
    }

    fn classification<I>(mut self, classification: I) -> BlackHoleBuilder<'a>
        where I: Into<Option<Type>> {

        self.classification = classification.into();
        self
    }

    /// Builds the black hole
    /// All members of BlackHoleBuilder are now moved to BlackHole
    ///
    ///
    fn build(self) -> BlackHole<'a> {
        BlackHole {

            name:               self.name,
            discovered_by:      self.discovered_by.unwrap_or(DEFAULT_DISCOVERED_BY),
            year_of_discovery:  self.year_of_discovery.unwrap_or(DEFAULT_DISCOVERED_YEAR),

            mass:               self.mass,
            angular_momentum:   self.angular_momentum,
            electric_charge:    self.electric_charge,

            classification:     self.classification
        }
    }
}

impl<'a> BlackHole<'a> {

    /// Calculate the radius of the event horizon, commonly referred as 'Schwarzschild radius'
    /// See: https://en.wikipedia.org/wiki/Schwarzschild_radius
    ///
    ///      2 * G * M
    /// rs = ---------
    ///        c ^ 2
    ///
    fn calc_event_horizon_radius(&self) -> Option<f64> {
        // TODO check the calculation
        self.mass.map(|mass| (2.0 * GRAVITATIONAL_CONSTANT * mass) / (SPEED_OF_LIGHT.pow(2) as f64))
    }
}

fn main() {

    let black_hole = BlackHoleBuilder::new("Gargantua")
        .discovered_by("Dr. Mann")
        .mass(123456789.0)
        .classification(None)
        .build();

    println!("Black hole {} has an event horizon radius of: {}", black_hole.name,
         black_hole.calc_event_horizon_radius().unwrap_or(0.0));

    // TODO documentation
    // TODO tests are still missing
    // TODO can we do anything about the unused compiler warnings? Use the stuff?
    // TODO create a crate?

    // -
    // the builder needs to have the same fields as the original struct
    // what if we need to pass in multiple must-have values - with name it was easy
    // what if we want to create multiple black holes with the same builder? (the black hole owns now the stuff) -> see blueprint
    // no real-life scenario (visibility) - BlackHole should be in a different crate
    // Adding of lifetimes makes code more difficult to read

    // +
    // Into works quite well
    // ownership is transferred from BlackHoleBuilder to BlackHole
    // Allows concise structuring of building process
    // Default values can be used (see discovered_by and year_of_discovery
    // By using Option<?> we can indicate that a value is optional and must not necessarily be set
    // String is now &str (However, we now must ensure that BlackHole doesn't outlive name and discovered_by
}