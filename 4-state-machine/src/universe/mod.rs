
extern crate test;

use std::f64;
use std::convert::From;

const INITIAL_NAME: &'static str = "Unknown";
const DEFAULT_DISCOVERED_YEAR: u16 = 2017;

/// The type of a black hole
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

/// This trait is used for the different states the builder of a black
/// hole can be in. Using this technique ensures that all the necessary
/// filds on the resulting black hole are set.
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

/// This is the builder of a black hole
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
 
    /// Constructs a new black hole builder
    /// 
    /// # Examples
    /// 
    /// ```
    /// let black_hole = BlackHole::new()
    ///     .name("Gargantua")
    ///     .discovered_by("Dr. Mann")
    ///     .year_of_discovery(2400)
    ///     .mass(123456789.0)
    ///     .classification(Type::SuperMassive)
    ///     .angular_momentum(12345.0)
    ///     .build();
    /// ```
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

    /// Sets the name of the black hole
    pub fn name(mut self, name: &str) -> BlackHoleBuilder<DiscoveredByBuilder> {
        self.black_hole.name = name.to_string();
        self.transition(DiscoveredByBuilder)
    }
}

impl BlackHoleBuilder<DiscoveredByBuilder> {

    /// Sets the name of the person/institution who discovered the black hole
    pub fn discovered_by(mut self, discovered_by: &str) -> BlackHoleBuilder<YearOfDiscoveryBuilder> {
        self.black_hole.discovered_by = discovered_by.to_string();
        self.transition(YearOfDiscoveryBuilder)
    }
}

impl BlackHoleBuilder<YearOfDiscoveryBuilder> {

    /// Sets the year of discovery of the black hole
    pub fn year_of_discovery(mut self, year_of_discovery: u16) -> BlackHoleBuilder<OptionalParamsBuilder> {
        self.black_hole.year_of_discovery = year_of_discovery;
        self.transition(OptionalParamsBuilder)
    }
}

impl BlackHoleBuilder<OptionalParamsBuilder> {

    /// Sets the mass of the black hole
    pub fn mass(mut self, mass: f64) -> Self {
        self.black_hole.mass = Some(mass);
        self
    }

    /// Sets the angular momentum of the black hole
    pub fn angular_momentum(mut self, angular_momentum: f64) -> Self {
        self.black_hole.angular_momentum = Some(angular_momentum);
        self
    }

    /// Sets the electric charge of the black hole
    pub fn electric_charge(mut self, electric_charge: f64) -> Self {
        self.black_hole.electric_charge = Some(electric_charge);
        self
    }

    /// Sets the type of the black hole
    pub fn classification(mut self, classification: Type) -> Self {
        self.black_hole.classification = Some(classification);
        self
    }

    /// Builds the black hole
    pub fn build(self) -> BlackHole {
        self.black_hole
    }

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
    ///
    /// assert_eq!(black_hole_1, black_hole_2);                   
    /// ```
    pub fn build_copy(&self) -> BlackHole {
        
        BlackHole {

            name:               self.black_hole.name.clone(),
            discovered_by:      self.black_hole.discovered_by.clone(),
            year_of_discovery:  self.black_hole.year_of_discovery,

            mass:               self.black_hole.mass,
            angular_momentum:   self.black_hole.angular_momentum,
            electric_charge:    self.black_hole.electric_charge,

            classification:     self.black_hole.classification.clone()
        }
    }
}

impl BlackHole {

    pub fn new() -> BlackHoleBuilder<NameBuilder> {
        BlackHoleBuilder::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use self::test::Bencher;

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

    #[bench]
    fn bench_create_black_hole(b: &mut Bencher) {
        b.iter (|| BlackHole::new()
            .name("Gargantua")
            .discovered_by("Dr. Mann")
            .year_of_discovery(2400)
            .mass(123456789.0)
            .classification(Type::SuperMassive)
            .electric_charge(2345.6)
            .angular_momentum(12345.0)
            .build());
    }
}
