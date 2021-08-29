use aoc2019::get_input;

fn main() -> Result<(),ureq::Error> {
	println!("Day 2");
	let input = get_input(2)?;
	let base_machine = aoc2019::intcode::Machine::from_file(input);
	let mut machine = base_machine.clone();
	machine.set_mem(1, 12);
	machine.set_mem(2, 2);
	machine.run(None).expect("Error during execution");

	println!("First solution: {}", machine.get_mem(0));
	for noun in 0..=99 {
		for verb in 0..=99 {
			let mut machine = base_machine.clone();
			machine.set_mem(1, noun);
			machine.set_mem(2, verb);
			machine.run(None).expect("Error during execution");
			if machine.get_mem(0) == 19690720 {
				println!("Second solution: {}", 100 * noun + verb);
				return Ok(());
			}
		}
	}
	Ok(())
}
