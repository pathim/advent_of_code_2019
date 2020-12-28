use aoc2019::get_input;
use aoc2019::intcode::SimpleInOut;

fn main() -> std::io::Result<()> {
	println!("Day 9");
	let input = get_input(9)?;
	let base_machine = aoc2019::intcode::Machine::from_file(input);

	let mut machine = base_machine.clone();
	let first = *machine
		.run(SimpleInOut::new(1))
		.expect("Error during execution").out_values
		.last()
		.expect("No output");
	println!("First solution: {}", first);

	let mut machine = base_machine.clone();
	let second = *machine
		.run(SimpleInOut::new(2))
		.expect("Error during execution").out_values
		.last()
		.expect("No output");
	println!("Second solution: {}", second);
	Ok(())
}
