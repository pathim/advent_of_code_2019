use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};

fn main() -> std::io::Result<()> {
	let day = 21;
	let input = get_input(day)?;
	let mut m = Machine::from_file(input);

	let mut cmd = String::new();
	let stdin = std::io::stdin();

	let cmd1 = b"NOT A T
	OR T J
	NOT B T
	OR T J
	NOT C T
	OR T J
	AND D J
	WALK
	"
	.iter()
	.map(|&x| x as Int);
	let (_, out) = m.clone().run(cmd1).expect("Error during execution");
	println!("result1: {}", out.last().unwrap());

	loop {
		let cmd_in: Vec<_> = cmd.as_bytes().iter().map(|x| *x as Int).collect();
		cmd.clear();
		let (fin, out) = m.run(cmd_in).expect("Error during execution");
		let out: Vec<_> = out.iter().map(|x| *x as u8).collect();
		print!("{}", String::from_utf8_lossy(&out));
		if fin {
			return Ok(());
		}
		stdin.read_line(&mut cmd)?;
	}
	Ok(())
}
