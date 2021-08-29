use aoc2019::get_input;

fn main() -> Result<(),ureq::Error> {
	println!("Day 5");
	let input = get_input(5)?;
	let base_machine = aoc2019::intcode::Machine::from_file(input);

	let mut machine = base_machine.clone();
	let first = *machine
		.run(Some(1))
		.expect("Error during execution")
		.1
		.last()
		.expect("No output");
	println!("First solution: {}", first);

	let mut machine = base_machine.clone();
	let second = *machine
		.run(Some(5))
		.expect("Error during execution")
		.1
		.last()
		.expect("No output");
	println!("Second solution: {}", second);
	Ok(())
}
