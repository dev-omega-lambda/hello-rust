mod r#box;
mod csv;

use std::time::{Duration, Instant};

use num::Complex;

use crate::csv::penguin;
use crate::r#box::cereal;

fn greet_world() {
    println!("Hello world!");
    let japan = "japan";
    let germany_b = "germany";
    let regions = [germany_b, japan];

    for region in IntoIterator::into_iter(regions) {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
    penguin();
    cereal();

    let a = 10;
    if a == 10 {
        println!("Variable \"A\" is equal Ten\n");
    }

    let complex_a = Complex { re: 2.1, im: -1.2 };
    let complex_b = Complex::new(11.1, 22.2);
    let add_result = complex_a + complex_b;
    println!("{} + {}i \n", add_result.re, add_result.im);

    for i in 0..5 {
        print!("Anonymous -> {} --", &i);
        if i == 4 {
            println!();
        }
    }

    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("\nCount is {}", count);
}
