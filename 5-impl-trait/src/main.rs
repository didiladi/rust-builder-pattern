
#![feature(conservative_impl_trait, test)]
#![allow(warnings)]
pub mod universe;

use universe::{BlackHole, NameBuilder, DiscoveredByBuilder, BlackHoleBuilder};

fn main() {

    let black_hole = BlackHole::new()
        .name("Gargantua")
        .discovered_by("Dr. Mann")
        .year_of_discovery(2030)
        .mass(123456789.0)
        .angular_momentum(12345.0)
        .build();
}
