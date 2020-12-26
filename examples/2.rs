use aoc2019::get_input;
use std::io::Read;

fn main()->std::io::Result<()>{
	println!("Day 2");
	let mut input=get_input(2)?;
	let mut buf=String::new();
	input.read_to_string(&mut buf)?;
	let mut mem=aoc2019::intcode::init_mem(&buf);
	mem[1]=12;
	mem[2]=2;
	aoc2019::intcode::execute(&mut mem).expect("Error during execution");
	println!("First solution: {}",mem[0]);
	for noun in 0..=99{
		for verb in 0..=99{
			let mut mem=aoc2019::intcode::init_mem(&buf);
			mem[1]=noun;
			mem[2]=verb;
			aoc2019::intcode::execute(&mut mem).expect("Error during execution");
			if mem[0]==19690720{
				println!("Second solution: {}",100*noun+verb);
				return Ok(())

			}
		}
	}
	Ok(())
}