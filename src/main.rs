mod csv;

use crate::csv::penguin;

fn greet_world() {
    println!("Hello world!");
    let japan = "japan";
    let germany = "germany";
    let regions = [germany, japan];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
    penguin();
}
