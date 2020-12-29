use aoc2019::get_input;
use aoc2019::intcode::Machine;

fn neighbours(x: i64, y: i64) -> [(i64, i64); 5] {
	[(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1), (x, y)]
}

fn main() -> std::io::Result<()> {
	let day = 17;
	let input = get_input(day)?;
	let mut m = Machine::from_file(input);
	let (_, res) = m.run(None).expect("Error during execution");
	let lines = res
		.split(|x| *x == 10)
		.map(|x| Vec::from(x))
		.filter(|x| x.len() > 0)
		.collect::<Vec<_>>();
	let mut first = 0;
	for x in 0..lines[0].len() {
		for y in 0..lines.len() {
			let nb = neighbours(x as i64, y as i64);
			let nb_count: i32 = nb
				.iter()
				.filter_map(|(a, b)| {
					if *a < 0 || *b < 0 {
						None
					} else {
						Some((*a as usize, *b as usize))
					}
				})
				.filter_map(|(a, b)| lines.get(b).map(|x| (a, x)))
				.filter_map(|(a, l)| l.get(a))
				.map(|x| (*x == 35) as i32)
				.sum();
			if nb_count == 5 {
				first += x * y;
			}
		}
	}
	for line in lines.iter() {
		for c in line.iter() {
			print!("{}", (*c as u8) as char);
		}
		println!("");
	}
	println!("First solution {}", first);

	Ok(())
}
