
#![feature(test)]
pub mod universe;

use universe::{BlackHole, Type};

fn main() {

    let black_hole = BlackHole::new()
        .name("Gargantua")
        .discovered_by("Dr. Mann")
        .year_of_discovery(2400)
        .mass(123456789.0)
        .classification(Type::SuperMassive)
        .build();
}
