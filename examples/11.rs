use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};
use num_complex::Complex;
use std::collections::HashMap;

struct PaintRobot {
	pos: Complex<i64>,
	dir: Complex<i64>,
	painted: HashMap<Complex<i64>, Int>,
	default_color: Int,
}

impl PaintRobot {
	fn new(default_color: Int) -> Self {
		Self {
			pos: Complex::new(0, 0),
			dir: Complex::new(0, 1),
			painted: HashMap::new(),
			default_color,
		}
	}
	fn count_painted(&self) -> usize {
		self.painted.len()
	}
	fn get_color(&self) -> Int {
		*self.painted.get(&self.pos).unwrap_or(&self.default_color)
	}
	fn input(&mut self, input: Vec<Int>) {
		for i in input.chunks_exact(2) {
			self.paint(i[0]);
			self.turn(i[1]);
		}
	}
	fn turn(&mut self, dir: Int) {
		self.dir *= Complex::i() * if dir == 1 { -1 } else { 1 };
		self.pos += self.dir;
	}
	fn paint(&mut self, color: Int) {
		self.painted.insert(self.pos, color);
	}
}

impl std::fmt::Display for PaintRobot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let max_re = self
			.painted
			.keys()
			.map(|x| x.re)
			.chain(Some(self.pos.re))
			.max()
			.unwrap_or(0);
		let max_im = self
			.painted
			.keys()
			.map(|x| x.im)
			.chain(Some(self.pos.im))
			.max()
			.unwrap_or(0);
		let min_re = self
			.painted
			.keys()
			.map(|x| x.re)
			.chain(Some(self.pos.re))
			.min()
			.unwrap_or(0);
		let min_im = self
			.painted
			.keys()
			.map(|x| x.im)
			.chain(Some(self.pos.im))
			.min()
			.unwrap_or(0);

		let c = if self.dir == Complex::new(1, 0) {
			">"
		} else if self.dir == Complex::new(-1, 0) {
			"<"
		} else if self.dir == Complex::i() {
			"^"
		} else {
			"v"
		};

		for y in -max_im..=-min_im {
			for x in min_re..=max_re {
				let current = Complex::new(x, -y);
				if current == self.pos {
					f.write_str(c)?;
					continue;
				}
				let char = self
					.painted
					.get(&current)
					.map_or(" ", |x| if *x == 1 { "#" } else { " " });
				f.write_str(char)?;
			}
			f.write_str("\n")?;
		}
		Ok(())
	}
}

fn main() -> Result<(),ureq::Error> {
	let day = 11;
	println!("Day {}", day);
	let base_machine = Machine::from_file(get_input(day)?);
	let mut m1 = base_machine.clone();
	let mut robot1 = PaintRobot::new(0);
	loop {
		let input = robot1.get_color();
		let (finished, res) = m1.run(Some(input)).expect("Error during execution");
		robot1.input(res);
		if finished {
			break;
		}
	}

	let first = robot1.count_painted();
	println!("First solution {}", first);

	let mut m2 = base_machine.clone();
	let mut robot2 = PaintRobot::new(1);
	loop {
		let input = robot2.get_color();
		let (finished, res) = m2.run(Some(input)).expect("Error during execution");
		robot2.input(res);
		if finished {
			break;
		}
	}
	println!("Second solution");
	print!("{}", robot2);

	Ok(())
}
