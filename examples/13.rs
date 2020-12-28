use aoc2019::get_input;
use aoc2019::intcode::Machine;
fn main() -> std::io::Result<()> {
	let day = 13;
	println!("Day {}", day);
	let base_machine = Machine::from_file(get_input(day)?);
	let mut m = base_machine.clone();
	let (_finished, res) = m.run(None).expect("Error");
	let first = res.chunks_exact(3).filter(|x| x[2] == 2).count();
	println!("First solution {}", first);
	Ok(())
}
