use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};
use itertools::Itertools;

fn find_output(phases: Vec<Int>, base_m: Machine) -> Int {
	let mut machines = Vec::new();
	for p in phases {
		let mut m = base_m.clone();
		let res = m.run(Some(p)).expect("Error during execution");
		machines.push(m);
		if res.1.len() != 0 {
			panic!("Output too soon");
		}
	}
	let mut input = 0;
	loop {
		let mut finished = true;
		for m in machines.iter_mut() {
			let res = m.run(Some(input)).expect("Error during execution");
			if !res.0 {
				finished = false;
			}
			input = *res.1.last().expect("No output");
		}
		if finished {
			return input;
		}
	}
}
fn main() -> std::io::Result<()> {
	let day = 7;
	println!("Day {}", day);
	let base_machine = Machine::from_file(get_input(day)?);
	let phases = 0..=4;
	let mut max_out = 0;
	for mut p in phases.permutations(5) {
		let mut input = 0;
		let out = loop {
			let mut m = base_machine.clone();
			if let Some(phase) = p.pop() {
				let mut a = Vec::new();
				a.push(phase);
				a.push(input);
				let res = m.run(a).expect("Error during execution");
				input = res.1[0]
			} else {
				break input;
			}
		};
		if out > max_out {
			max_out = out;
		}
	}
	println!("First solution {}", max_out);

	let phases = 5..=9;
	let mut max_out = 0;
	for p in phases.permutations(5) {
		let bm = base_machine.clone();
		let out = find_output(p, bm);
		if out > max_out {
			max_out = out;
		}
	}

	println!("Second solution {}", max_out);
	Ok(())
}
