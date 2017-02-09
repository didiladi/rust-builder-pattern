
const UNKNOWN: &'static str = "Unknown";
const DEFAULT_DISCOVERED_YEAR: u16 = 2017;

/// the type of a black hole
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    SuperMassive,
    IntermediateMassive,
    Stellar,
    Micro
}

/// The black hole
///
/// # Examples
///
/// You can construct a black hole by using the builder pattern:
/// 
/// ```
/// let black_hole = BlackHole::new()
///     .name("Gargantua")
///     .discovered_by("Dr. Mann")
///     .year_of_discovery(2400)
///     .mass(123456789.0)
///     .classification(Type::SuperMassive)
///     .build();
/// ```
pub struct BlackHole {

    /// The name of the black hole
    pub name: String,

    /// The name of the person who discovered the black hole
    pub discovered_by: String,

    /// The year when the black hole was discovered
    pub year_of_discovery: u16,

    /// The mass of the black hole
    pub mass: Option<f64>,

    /// The angular momentum (how much does it spin?)
    pub angular_momentum: Option<f64>,

    /// The electric charge
    pub electric_charge: Option<f64>,

    /// The type of the black hole
    pub classification: Option<Type>
}

impl BlackHole {

    /// Constructs a new builder for a black hole
    pub fn new() -> NameBuilder {
        NameBuilder {
            black_hole: BlackHole {
                name:               UNKNOWN.to_string(),
                discovered_by:      UNKNOWN.to_string(),
                year_of_discovery:  DEFAULT_DISCOVERED_YEAR,
                mass:               None,
                angular_momentum:   None,
                electric_charge:    None,
                classification:     None
            }
        }
    }
}

pub struct NameBuilder {
    black_hole: BlackHole
}

impl NameBuilder {

    /// Sets the name of the black hole. Returns a DiscoveredByBuilder.
    pub fn name(mut self, name: &str) -> impl DiscoveredByBuilder {
        self.black_hole.name = name.to_string();
        self
    }
}

pub trait DiscoveredByBuilder {

    /// Sets the name of the person who discovered the black hole. Returns a
    /// YearOfDiscoveryBuilder.
    fn discovered_by(mut self, discovered_by: &str) -> YearOfDiscoveryBuilder;
}

impl DiscoveredByBuilder for NameBuilder {
    fn discovered_by(mut self, discovered_by: &str) -> YearOfDiscoveryBuilder {
        self.black_hole.discovered_by = discovered_by.to_string();
        YearOfDiscoveryBuilder {
            black_hole: self.black_hole
        }
    }
}

pub struct YearOfDiscoveryBuilder {
    black_hole: BlackHole
}

impl YearOfDiscoveryBuilder {
    
    /// Sets the year of discovery of the black hole. Returns a BlackHoleBuilder, which enables you
    /// to set a bunch of optional fields on the black hole
    pub fn year_of_discovery(mut self, year_of_discovery: u16) -> impl BlackHoleBuilder {
        self.black_hole.year_of_discovery = year_of_discovery;
        self
    }
}

/// This builder enables one to set a bunch of optional fields for the balck hole.
pub trait BlackHoleBuilder {
    
    /// Sets the mass of the black hole
    fn mass(mut self, mass: f64) -> Self;

    /// Sets the angular momentum of the black hole
    fn angular_momentum(mut self, angular_momentum: f64) -> Self;

    /// Sets the electric charge of the black hole
    fn electric_charge(mut self, electric_charge: f64) -> Self;

    /// Sets the type of the black hole
    fn classification(mut self, classification: Type) -> Self;

    /// Build the black hole
    fn build(self) -> BlackHole;

    /// Builds a copy of the black hole. This function does not take ownership
    /// of the builder and therefore enables you to build multiple black holes 
    /// by re-using the same builder.
    ///
    /// # Examples
    /// 
    /// ```
    /// let builder = BlackHole::new()
    ///     .name("Gargantua")
    ///     .discovered_by("Dr. Mann")
    ///     .year_of_discovery(2400)
    ///     .mass(123456789.0)
    ///     .classification(Type::SuperMassive);
    ///
    /// let black_hole_1 = builder.build_copy();
    /// let black_hole_2 = builder.classification(Type::Massive)
    ///                           .build_copy();
    /// ```
    fn build_copy(&self) -> BlackHole;
}

impl BlackHoleBuilder for YearOfDiscoveryBuilder {
    
    fn mass(mut self, mass: f64) -> Self {
        self.black_hole.mass = Some(mass);
        self
    }

    fn angular_momentum(mut self, angular_momentum: f64) -> Self {
        self.black_hole.angular_momentum = Some(angular_momentum);
        self
    }

    fn electric_charge(mut self, electric_charge: f64) -> Self {
        self.black_hole.electric_charge = Some(electric_charge);
        self
    }

    fn classification(mut self, classification: Type) -> Self {
        self.black_hole.classification = Some(classification);
        self
    }

    fn build(self) -> BlackHole {
        self.black_hole
    }

    fn build_copy(&self) -> BlackHole {
        BlackHole {
        
            name:               self.black_hole.name.clone(),
            discovered_by:      self.black_hole.discovered_by.clone(),
            year_of_discovery:  self.black_hole.year_of_discovery,
            
            mass:               self.black_hole.mass,
            angular_momentum:   self.black_hole.angular_momentum,
            electric_charge:    self.black_hole.electric_charge,
            
            classification:      self.black_hole.classification.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_build() {

        let black_hole = BlackHole::new()
            .name("Gargantua")
            .discovered_by("Dr. Mann")
            .year_of_discovery(2400)
            .mass(123456789.0)
            .classification(Type::SuperMassive)
            .electric_charge(2345.6)
            .angular_momentum(12345.0)
            .build();

        assert_eq!("Gargantua", black_hole.name);
        assert_eq!("Dr. Mann", black_hole.discovered_by);
        assert_eq!(2400, black_hole.year_of_discovery);
        assert_eq!(Some(123456789.0), black_hole.mass);
        assert_eq!(Some(Type::SuperMassive), black_hole.classification);
        assert_eq!(Some(2345.6), black_hole.electric_charge);
        assert_eq!(Some(12345.0), black_hole.angular_momentum);
    }

    #[test]
    fn new_with_copy() {

        let builder = BlackHole::new()
            .name("Gargantua")
            .discovered_by("Dr. Mann")
            .year_of_discovery(2400)
            .mass(123456789.0)
            .classification(Type::SuperMassive)
            .electric_charge(2345.6)
            .angular_momentum(12345.0);

        let black_hole_1 = builder.build_copy();
        let black_hole_2 = builder.build_copy();

        assert_eq!(black_hole_1.name, black_hole_2.name);
        assert_eq!(black_hole_1.discovered_by, black_hole_2.discovered_by);
        assert_eq!(black_hole_1.year_of_discovery, black_hole_2.year_of_discovery);
        assert_eq!(black_hole_1.mass, black_hole_2.mass);
        assert_eq!(black_hole_1.classification, black_hole_2.classification);
        assert_eq!(black_hole_1.electric_charge, black_hole_2.electric_charge);
        assert_eq!(black_hole_1.angular_momentum, black_hole_2.angular_momentum);
    }
}


