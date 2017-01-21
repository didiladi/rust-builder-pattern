
pub mod universe;

use universe::{BlackHole, Type, NameBuilder};

fn main() {

    let black_hole = BlackHole::new()
        .name("Gargantua".to_string())
        .discovered_by("Dr. Mann".to_string())
        .year_of_discovery(2400)
        .mass(123456789.0)
        .classification(Type::SuperMassive)
        .build();
    
    println!("Black hole {} has an event horizon radius of: {}", black_hole.name(),
    black_hole.calc_event_horizon_radius().unwrap_or(0.0));

    // TODO documentation
    // TODO tests are still missing
    // TODO can we do anything about the unused compiler warnings? Use the stuff?
    // TODO create a crate?

    // -
    //

    // +
    //
}
