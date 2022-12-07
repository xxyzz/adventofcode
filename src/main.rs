#![feature(iter_array_chunks)]

mod day1;
mod day2;
mod day3;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u32 = (&args[1]).parse().expect("Not a number");
    let part: u32 = (&args[2]).parse().expect("Not a number");

    match day {
	1 => day1::resolve(part),
	2 => day2::resolve(part),
	3 => day3::resolve(part),
	_ => (),
    }
}
