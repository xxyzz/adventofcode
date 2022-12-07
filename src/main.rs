mod day1;
mod day2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u32 = (&args[1]).parse().expect("Not a number");
    let part: u32 = (&args[2]).parse().expect("Not a number");

    match day {
	1 => day1::resolve(part),
	2 => day2::resolve(part),
	_ => (),
    }
}
