use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};

fn main() -> std::io::Result<()> {
	let day = 25;
	let input = get_input(day)?;
	let mut m = Machine::from_file(input);

	let mut cmd=String::new();
	let stdin=std::io::stdin();

	loop{
		let cmd_in:Vec<_>=cmd.as_bytes().iter().map(|x| *x as Int).collect();
		cmd.clear();
		let (fin,out)=m.run(cmd_in).expect("Error during execution");
		let out:Vec<_>=out.iter().map(|x| *x as u8).collect();
		print!("{}",String::from_utf8_lossy(&out));
		if fin{
			return Ok(());
		}
		stdin.read_line(&mut cmd)?;
	}
}