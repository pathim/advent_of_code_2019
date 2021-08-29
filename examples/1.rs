use aoc2019::get_input;
use std::io::{BufRead, BufReader};

fn fuel_need(input: u64) -> u64 {
	(input / 3).saturating_sub(2)
}

fn fuel_sum(input: u64) -> u64 {
	if input == 0 {
		return 0;
	}
	input + fuel_sum(fuel_need(input))
}

fn total_fuel_need(input: u64) -> u64 {
	fuel_sum(fuel_need(input))
}

fn main() -> Result<(),ureq::Error> {
	let input = BufReader::new(get_input(1)?);
	let mass = input
		.lines()
		.map(|l| l.unwrap().parse::<u64>().unwrap())
		.collect::<Vec<u64>>();
	println!(
		"First result: {}",
		mass.iter().map(|&m| fuel_need(m)).sum::<u64>()
	);
	println!(
		"Second result: {}",
		mass.iter().map(|&m| total_fuel_need(m)).sum::<u64>()
	);
	Ok(())
}
