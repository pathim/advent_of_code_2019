use aoc2019::get_input;

fn main() -> std::io::Result<()> {
	println!("Day 5");
	let input = get_input(5)?;
	let base_machine = aoc2019::intcode::Machine::from_file(input);

	let mut machine = base_machine.clone();
	let inout=aoc2019::intcode::SimpleInOut::new(1);
	let first=*machine
		.run(inout)
		.expect("Error during execution").out_values.last()
		.expect("No output");
	println!("First solution: {}", first);

	let mut machine = base_machine.clone();
	let inout=aoc2019::intcode::SimpleInOut::new(5);
	let second=*machine
		.run(inout)
		.expect("Error during execution").out_values.last()
		.expect("No output");
	println!("Second solution: {}", second);
	Ok(())
}
