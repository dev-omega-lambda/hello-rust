mod r#box;
mod csv;

use crate::csv::penguin;
use crate::r#box::cereal;

fn greet_world() {
    println!("Hello world!");
    let japan = "japan";
    let germany_b = "germany";
    let regions = [germany_b, japan];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
    penguin();
    cereal();

    let a = 10;
    if a == 10 {
        println!("Variable \"A\" is equal Ten");
    }
}
