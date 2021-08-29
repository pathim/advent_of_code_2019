use aoc2019::get_input;
use std::{
	collections::{HashMap, HashSet},
	io::Read,
};

fn generate_bitmask(bit: u8) -> u32 {
	let mut res = 0;
	let deltas = [-5, -1, 1, 5];
	for d in deltas {
		if bit % 5 == 0 && d == -1 {
			continue;
		}
		if bit % 5 == 4 && d == 1 {
			continue;
		}
		let bitpos = (bit as i32) + d;
		if bitpos < 0 || bitpos >= 25 {
			continue;
		}
		res |= 1 << bitpos;
	}
	res
}

fn next_generation(current: u32, bitmasks: &Vec<u32>) -> u32 {
	let mut res = 0;
	for (i, m) in bitmasks.iter().enumerate() {
		let popcount = (current & m).count_ones();
		let bit = current & (1u32 << i);
		res |= if (bit != 0 && popcount == 1) || (bit == 0 && (popcount == 2 || popcount == 1)) {
			1
		} else {
			0
		} << i;
	}
	res
}
fn next_generation_fractal(current: u32, above: u32, below: u32, bitmasks: &Vec<u32>) -> u32 {
	let mut res = 0;
	for (i, m) in bitmasks.iter().enumerate() {
		if i == 12 {
			continue;
		}
		let mut popcount = (current & m).count_ones();
		if i % 5 == 0 {
			popcount += (above & (1 << 11) != 0) as u32;
		}
		if i % 5 == 4 {
			popcount += (above & (1 << 13) != 0) as u32;
		}
		if i <= 4 {
			popcount += (above & (1 << 7) != 0) as u32;
		}
		if i >= 20 {
			popcount += (above & (1 << 17) != 0) as u32;
		}
		if i == 7 {
			popcount += (below & 0b11111).count_ones();
		}
		if i == 17 {
			popcount += (below & 0b11111 << 20).count_ones();
		}
		if i == 11 {
			popcount += (below & 0b0000100001000010000100001).count_ones();
		}
		if i == 13 {
			popcount += (below & 0b1000010000100001000010000).count_ones();
		}
		let bit = current & (1u32 << i);
		res |= if (bit != 0 && popcount == 1) || (bit == 0 && (popcount == 2 || popcount == 1)) {
			1
		} else {
			0
		} << i;
	}
	res
}

fn main() -> Result<(),ureq::Error> {
	let mut input = get_input(24)?;
	let mut buf = Vec::new();
	input.read_to_end(&mut buf)?;
	let mut startval = 0u32;
	let mut cval = 1;
	for c in buf {
		match c {
			b'.' => cval *= 2,
			b'#' => {
				startval |= cval;
				cval *= 2
			}
			_ => {}
		}
	}
	let startval = startval;
	let bitmasks: Vec<u32> = (0..25).map(generate_bitmask).collect();
	let mut seen = HashSet::new();
	let mut area = startval;
	let result1 = loop {
		if seen.contains(&area) {
			break area;
		}
		seen.insert(area);
		area = next_generation(area, &bitmasks);
	};
	println!("Result 1: {}", result1);

	let mut layers = HashMap::new();
	layers.insert(0, startval);

	for lmax in 0..200 {
		let cl = layers.clone();
		for li in -(lmax + 1)..=lmax + 1 {
			let new_layer = next_generation_fractal(
				*cl.get(&li).unwrap_or(&0),
				*cl.get(&(li + 1)).unwrap_or(&0),
				*cl.get(&(li - 1)).unwrap_or(&0),
				&bitmasks,
			);
			layers.insert(li, new_layer);
		}
	}
	let result2: u32 = layers.values().map(|x| x.count_ones()).sum();
	println!("Result 2: {}", result2);

	Ok(())
}
