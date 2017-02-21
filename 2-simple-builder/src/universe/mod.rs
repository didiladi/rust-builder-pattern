
extern crate test;

use std::f64;

const DEFAULT_DISCOVERED_BY: &'static str = "Unknown";
const DEFAULT_DISCOVERED_YEAR: u16 = 2017;

// The type of a black hole
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
/// ```
/// You can construct a black hole by using the builder pattern
///
/// let black_hole = BlackHoleBuilder::new("Gargantua")
///	.discovered_by("Dr. Mann".to_string())
///	.mass(123456789.0)
///	.classification(None)
///	.build();
/// ```
#[derive(Debug, PartialEq)]
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

/// The builder for the black hole. It contains all properties of the
/// black hole. Upon creation, the set properties are copied (or cloned)
/// into BlackHole
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

    /// Constructs a new BlackHoleBuilder
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

    /// Sets the name of the person/institution who discovered the balck hole
    pub fn discovered_by<I>(mut self, discovered_by: I) -> BlackHoleBuilder
        where I: Into<Option<String>> {

        self.discovered_by = discovered_by.into();
        self
    }

    /// Sets the year of discovery
    pub fn year_of_discovery<I>(mut self, year_of_discovery: I) -> BlackHoleBuilder
        where I: Into<Option<u16>> {

        self.year_of_discovery = year_of_discovery.into();
        self
    }

    /// Sets the mass
    pub fn mass<I>(mut self, mass: I) -> BlackHoleBuilder
        where I: Into<Option<f64>> {

        self.mass = mass.into();
        self
    }

    /// Sets the angular momentum
    pub fn angular_momentum<I>(mut self, angular_momentum: I) -> BlackHoleBuilder
        where I: Into<Option<f64>> {

        self.angular_momentum = angular_momentum.into();
        self
    }

    /// Sets the electric charge
    pub fn electric_charge<I>(mut self, electric_charge: I) -> BlackHoleBuilder
        where I: Into<Option<f64>> {

        self.electric_charge = electric_charge.into();
        self
    }

    /// Sets the type of the black hole
    pub fn classification<I>(mut self, classification: I) -> BlackHoleBuilder
        where I: Into<Option<Type>> {

        self.classification = classification.into();
        self
    }

    /// Builds the black hole
    ///
    /// All members of BlackHoleBuilder are now moved to the resulting BlackHole and
    /// their ownership is transferred (the created BlackHole now owns the data
    /// of the builder)
    /// 
    /// # Examples
    /// 
    /// ```
    /// let black_hole = BlackHoleBuilder::new("Gargantua")
    ///	    .discovered_by("Dr. Mann".to_string())
    ///	    .mass(123456789.0)
    ///	    .classification(None)
    ///	    .build();
    /// ```
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

    /// Builds a copy of the black hole
    ///
    /// Here all data from the builder is cloned into the new BlackHole. No ownership
    /// is transferred (the builder still owns all the data). This method is intended
    /// to be used whenever you want to create multiple black holes which have similar
    /// properties.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let builder = BlackHoleBuilder::new("Gargantua")
    ///     .discovered_by("Dr. Mann".to_string())
    ///     .mass(123456.0)
    ///     .classification(None)
    ///     .angular_momentum(1234.5);
    ///
    /// let black_hole_1 = builder.build_copy();
    /// let black_hole_2 = builder
    ///     .electric_charge(56789.1)
    ///     .build();
    /// 
    /// assert_eq!(black_hole_1, black_hole_2);
    /// ```
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
}


#[cfg(test)]
mod tests {
    
    use super::*;
    use self::test::Bencher;

    #[test]
    fn new_with_build() {

        let black_hole = BlackHoleBuilder::new("Gargantua")
            .discovered_by("Dr. Mann".to_string())
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

        let builder = BlackHoleBuilder::new("Gargantua")
            .discovered_by("Dr. Mann".to_string())
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
    
    #[bench]
    fn bench_create_black_hole(b: &mut Bencher) {
        b.iter (|| BlackHoleBuilder::new("Gargantua")
            .discovered_by("Dr. Mann".to_string())
            .year_of_discovery(2400)
            .mass(123456789.0)
            .classification(Type::SuperMassive)
            .electric_charge(2345.6)
            .angular_momentum(12345.0)
            .build());
    }
}
