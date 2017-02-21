
use std::f64;

const GRAVITATIONAL_CONSTANT: f64 = 0.00000000006674; // N*m^2/kg^2
const SPEED_OF_LIGHT: u64 = 299_792_458; // m/s

enum Type {

    SuperMassive,
    IntermediateMassive,
    Stellar,
    Micro
}

struct BlackHole {

    name: String,
    discovered_by: String,
    year_of_discovery: u16,
    mass: f64,
    angular_momentum: f64,
    electric_charge: f64,
    classification: Type
}

impl BlackHole {

    fn new(name: String, discovered_by: String, mass: f64, angular_momentum: f64,
           electric_charge: f64, year_of_discovery: u16, classification: Type) -> BlackHole {

        BlackHole {
            name:               name,
            discovered_by:      discovered_by,
            year_of_discovery:  year_of_discovery,
            mass:               mass,
            angular_momentum:   angular_momentum,
            electric_charge:    electric_charge,
            classification:     classification
        }
    }

    /// Calculate the radius of the event horizon, commonly referred as 'Schwarzschild radius'
    /// See: https://en.wikipedia.org/wiki/Schwarzschild_radius
    ///
    ///      2 * G * M
    /// rs = ---------
    ///        c ^ 2
    ///
    fn calc_event_horizon_radius(&self) -> f64 {

        // TODO this would be perfect as const -> but we can't do that (compiler error) -> Can we at least make sure that the values are the same?
        let check: f64 = 6.674_f64.powf(-11.0);
        assert_eq!(check, GRAVITATIONAL_CONSTANT);
        // TODO check the calculation
        (2.0 * GRAVITATIONAL_CONSTANT * self.mass) / (SPEED_OF_LIGHT.pow(2) as f64)
    }
}

fn main() {

    // first option:
    let gargantua = BlackHole {
        name:               "Gargantua".to_string(),
        discovered_by:      "Dr. Mann".to_string(),
        year_of_discovery:  2038,
        mass:               1234.0,
        angular_momentum:   3.874,
        electric_charge:    343.6,
        classification:     Type::SuperMassive
    };

    // second option
    let gargantua_clone = BlackHole::new("Gargantua".to_string(), "Dr. Mann".to_string(),
                                         1234.0, 3.874, 343.6, 2038, Type::SuperMassive);

    println!("Black hole {} has an event horizon radius of: {}", gargantua.name,
             gargantua.calc_event_horizon_radius());

    // TODO documentation
    // TODO tests are still missing
    // TODO can we do anything about the unused compiler warnings? Use the stuff?
    // TODO create a crate?
}