
pub mod universe;

use universe::{Type, BlackHoleBuilder};

fn main() {

    let black_hole = BlackHoleBuilder::new("Gargantua")
        .discovered_by("Dr. Mann".to_string())
        .mass(123456789.0)
        .classification(None)
        .build();

    println!("Black hole {} has an event horizon radius of: {}", black_hole.name,
         black_hole.calc_event_horizon_radius().unwrap_or(0.0));

    let black_hole_builder = BlackHoleBuilder::new("Gargantua")
        .discovered_by("Dr. Mann".to_string())
        .mass(123456789.0)
        .classification(None);

    let clone1 = black_hole_builder.build_copy();
    let clone2 = black_hole_builder.build_copy();
    // TODO equals


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
    // we can mess with the internals of BlackHole if we want

    // +
    // Into works quite well
    // ownership is transferred from BlackHoleBuilder to BlackHole
    // Allows concise structuring of building process
    // Default values can be used (see discovered_by and year_of_discovery
    // By using Option<?> we can indicate that a value is optional and must not necessarily be set
    // String is now &str (However, we now must ensure that BlackHole doesn't outlive name and discovered_by
    // for clone we just have to pass in a reference build_copy and derive the enum from Clone
}
