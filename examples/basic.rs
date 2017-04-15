extern crate floating_duration;

use std::time::Instant;

use floating_duration::{TimeAsFloat, TimeFormat};

fn main() {
    let start = Instant::now();

    let result = (1..12).fold(1, |acc, x| acc * x);

    let elapsed = start.elapsed();
    println!("Needed {}", TimeFormat(elapsed));
    println!("In seconds: {}", elapsed.as_fractional_secs());

    println!("Result: {}", result);
}
