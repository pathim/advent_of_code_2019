use aoc2019::get_input;
use itertools::Itertools;
use std::io::{BufReader, Read};

fn main() -> Result<(),ureq::Error> {
	let mut input = BufReader::new(get_input(8)?);
	const WIDTH: usize = 25;
	const HEIGHT: usize = 6;
	let mut result1 = 0;
	let mut zero_count = usize::MAX;
	let mut l = [0u8; HEIGHT * WIDTH];
	let mut image = [2u8; HEIGHT * WIDTH];
	while let Ok(_) = input.read_exact(&mut l) {
		for x in l.iter_mut() {
			*x -= b'0';
		}
		let c = l.iter().counts();
		let c0 = c[&0];
		if c0 < zero_count {
			zero_count = c0;
			result1 = c[&1] * c[&2];
		}
		for (pixel_result, pixel) in image.iter_mut().zip(l.iter()) {
			if *pixel_result == 2 {
				*pixel_result = *pixel;
			}
		}
	}
	println!("Result 1: {}", result1);
	println!("Result 2:");
	for line in image.chunks_exact(WIDTH) {
		println!(
			"{}",
			line.iter()
				.map(|x| if *x == 1 { '█' } else { ' ' })
				.join("")
		);
	}
	Ok(())
}
