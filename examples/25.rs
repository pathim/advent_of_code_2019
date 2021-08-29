use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;

use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};

fn interpret_move(data: &Vec<u8>) -> (Vec<Direction>, Vec<String>, bool) {
	let mut security = false;
	let mut lines = data.lines().filter_map(|x| x.ok());
	while let Some(l) = lines.next() {
		if l.starts_with("==") {
			security = l.starts_with("== Security");
			break;
		}
	}
	let mut lines = lines.skip_while(|x| !x.starts_with("Doors"));
	lines.next();
	let mut doors = Vec::new();
	while let Some(l) = lines.next() {
		if l.starts_with('-') {
			let ch: &[_] = &['-', ' '];
			doors.push(l.trim_matches(ch).into());
		} else {
			break;
		}
	}
	let mut lines = lines.skip_while(|x| !x.starts_with("Items"));
	lines.next();
	let mut items = Vec::new();
	for l in lines {
		if l.starts_with('-') {
			let ch: &[_] = &['-', ' '];
			items.push(l.trim_matches(ch).into());
		} else {
			break;
		}
	}
	(doors, items, security)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	N,
	S,
	E,
	W,
	Invalid,
}
impl From<&str> for Direction {
	fn from(s: &str) -> Self {
		if s == "north" {
			Self::N
		} else if s == "south" {
			Self::S
		} else if s == "east" {
			Self::E
		} else if s == "west" {
			Self::W
		} else {
			Self::Invalid
		}
	}
}

impl Display for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::N => "north",
			Self::S => "south",
			Self::E => "east",
			Self::W => "west",
			Self::Invalid => "invalid",
		};
		write!(f, "{}", s)
	}
}

impl Direction {
	pub fn opposite(&self) -> Self {
		match self {
			Self::N => Self::S,
			Self::S => Self::N,
			Self::E => Self::W,
			Self::W => Self::E,
			Self::Invalid => Self::Invalid,
		}
	}
}
struct Solver {
	m: Machine,
	position: Vec<Direction>,
	to_explore: HashMap<Vec<Direction>, Vec<Direction>>,
	finished: bool,
	item_loc: HashMap<String, Vec<Direction>>,
	security_loc: Vec<Direction>,
	security_dir: Direction,
}

impl Solver {
	pub fn new(input: File) -> Self {
		let mut m = Machine::from_file(input);
		let position = Vec::new();
		let mut to_explore = HashMap::new();
		let (finished, out) = m.run(None).expect("Error during execution");
		let out: Vec<_> = out.iter().map(|x| *x as u8).collect();
		let (doors, _, _) = interpret_move(&out);
		to_explore.insert(position.clone(), doors);
		Self {
			m,
			position,
			to_explore,
			finished,
			item_loc: HashMap::new(),
			security_loc: Vec::new(),
			security_dir: Direction::Invalid,
		}
	}

	pub fn map_and_collect(&mut self) {
		while !self.position.is_empty()
			|| !self
				.to_explore
				.get(&self.position)
				.map(|x| x.is_empty())
				.unwrap_or(true)
		{
			self.move_next();
		}
	}

	pub fn goto_security(&mut self) {
		while !self.position.is_empty() {
			self.go_back();
		}
		if self.security_loc.is_empty() {
			panic!("Security position not known");
		}
		let pos = self.security_loc.clone();
		for dir in pos {
			self.move_dir(dir);
		}
	}

	pub fn move_past_security(&mut self) {
		let inv: Vec<_> = self
			.command("inv\n")
			.lines()
			.filter(|x| x.starts_with('-'))
			.map(|x| x.split_at(2).1.to_owned())
			.collect();
		for i in 0..256 {
			for item in inv.iter() {
				self.command(&format!("drop {}\n", item));
			}
			for a in 0..8 {
				if (i >> a) & 1 == 1 {
					self.command(&format!("take {}\n", inv[a]));
				}
			}
			let move_result = self.command(&format!("{}\n", self.security_dir));
			let success = move_result
				.clone()
				.lines()
				.filter(|x| x.find("Alert!").is_some())
				.next()
				.is_none();
			if success {
				println!("{}", move_result);
				break;
			}
		}
	}

	fn move_next(&mut self) {
		let dir = self
			.to_explore
			.get(&self.position)
			.and_then(|x| x.first())
			.copied();
		match dir {
			Some(dir) => self.move_dir(dir),
			None => self.go_back(),
		}
	}

	pub fn command(&mut self, cmd: &str) -> String {
		let cmd_in: Vec<_> = cmd.as_bytes().iter().map(|x| *x as Int).collect();
		let (fin, out) = self.m.run(cmd_in).expect("Error during execution");
		self.finished = fin;
		let out: Vec<_> = out.iter().map(|x| *x as u8).collect();
		String::from_utf8_lossy(&out).into()
	}

	fn take(&mut self, item: &str) {
		if item != "infinite loop"
			&& item != "escape pod"
			&& item != "photons"
			&& item != "molten lava"
			&& item != "giant electromagnet"
		{
			self.item_loc
				.insert(item.to_string(), self.position.clone());
			let c = format!("take {}\n", item);
			let cmd_in: Vec<_> = c.as_bytes().iter().map(|x| *x as Int).collect();
			self.m.run(cmd_in).expect("Error during execution");
		}
	}

	fn move_dir(&mut self, dir: Direction) {
		self.to_explore
			.entry(self.position.clone())
			.and_modify(|x| x.retain(|v| *v != dir));
		let opposite = dir.opposite();
		if self.position.last() == Some(&opposite) {
			self.position.pop();
		} else {
			self.position.push(dir);
		}
		let cmd = format!("{}\n", dir);
		let cmd_in: Vec<_> = cmd.as_bytes().iter().map(|x| *x as Int).collect();
		let (fin, out) = self.m.run(cmd_in).expect("Error during execution");
		self.finished = fin;
		let out: Vec<_> = out.iter().map(|x| *x as u8).collect();
		let (mut doors, items, is_security) = interpret_move(&out);
		doors.retain(|x| *x != opposite);
		if is_security {
			self.security_loc = self.position.clone();
			self.security_dir = dir;
			doors.retain(|x| *x != dir);
		}
		let _doors = self
			.to_explore
			.entry(self.position.clone())
			.or_insert(doors);
		if let Some(item) = items.first() {
			self.take(item);
		}
	}

	fn go_back(&mut self) {
		let dir = self.position.last().map(|x| x.opposite());
		if let Some(dir) = dir {
			self.move_dir(dir);
		}
	}
}

fn main() -> Result<(), ureq::Error> {
	let day = 25;
	let input = get_input(day)?;
	let mut solver = Solver::new(input);

	solver.map_and_collect();
	solver.goto_security();
	solver.move_past_security();

	Ok(())
}
