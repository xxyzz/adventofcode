#![feature(iter_array_chunks)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u32 = (&args[1]).parse().expect("Not a number");
    let part: u32 = (&args[2]).parse().expect("Not a number");

    match day {
	1 => day1::resolve(part),
	2 => day2::resolve(part),
	3 => day3::resolve(part),
	4 => day4::resolve(part),
	5 => day5::resolve(part),
	6 => day6::resolve(part),
	7 => day7::resolve(part),
	_ => (),
    }
}
