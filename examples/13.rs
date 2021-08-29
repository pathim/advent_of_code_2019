use aoc2019::get_input;
use aoc2019::intcode::Machine;
fn main() -> Result<(),ureq::Error> {
	let day = 13;
	println!("Day {}", day);
	let base_machine = Machine::from_file(get_input(day)?);
	let mut m = base_machine.clone();
	let (_finished, res) = m.run(None).expect("Error");
	let first = res.chunks_exact(3).filter(|x| x[2] == 2).count();
	println!("First solution {}", first);

	let mut m = base_machine.clone();
	m.set_mem(0, 2);
	let mut input = 0;
	let mut ballx = 0;
	let mut paddle = 0;
	let mut score = 0;
	loop {
		let (finished, res) = m.run(Some(input)).expect("Error");
		for tile in res.chunks_exact(3) {
			if tile[0] == -1 {
				score = tile[2];
			} else if tile[2] == 4 {
				ballx = tile[0];
			} else if tile[2] == 3 {
				paddle = tile[0];
			};
		}
		input = (ballx - paddle).signum();
		if finished {
			break;
		}
	}
	println!("Second solution {}", score);
	Ok(())
}
