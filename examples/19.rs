use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};

fn is_beam(mut m: Machine, x: Int, y: Int) -> bool {
	let i = Vec::from([x, y]);
	let (_, res) = m.run(i).expect("Execution error");
	*res.first().expect("No output") == 1
}

fn main() -> std::io::Result<()> {
	let day = 19;
	let input = get_input(day)?;
	let bm = Machine::from_file(input);
	let mut first = 0;
	for y in 0..50 {
		for x in 0..50 {
			let val = is_beam(bm.clone(), x, y);
			first += val as i32;
		}
	}
	println!("First solution {}", first);

	Ok(())
}
