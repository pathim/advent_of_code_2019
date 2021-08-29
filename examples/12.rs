use std::io::{BufRead, BufReader};

use aoc2019::get_input;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
	x: i64,
	y: i64,
	z: i64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Velocity {
	x: i64,
	y: i64,
	z: i64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Gravity {
	x: i64,
	y: i64,
	z: i64,
}

impl Position {
	pub fn apply_velocity(&mut self, v: &Velocity) {
		self.x += v.x;
		self.y += v.y;
		self.z += v.z;
	}
	pub fn calc_gravity(&self, other: &Position) -> Gravity {
		Gravity {
			x: (other.x - self.x).signum(),
			y: (other.y - self.y).signum(),
			z: (other.z - self.z).signum(),
		}
	}
	pub fn energy(&self) -> i64 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
}

impl Velocity {
	pub fn apply_gravity(&mut self, g: &Gravity) {
		self.x += g.x;
		self.y += g.y;
		self.z += g.z;
	}
	pub fn energy(&self) -> i64 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
	pos: Position,
	vel: Velocity,
}

impl Moon {
	pub fn from_line(l: &str) -> Moon {
		let vel = Velocity { x: 0, y: 0, z: 0 };
		let mut it = l[..l.len() - 1]
			.split(|x| x == ',')
			.map(|x| x.split_once('=').unwrap().1.trim())
			.map(|x| x.parse().unwrap());
		let pos = Position {
			x: it.next().unwrap(),
			y: it.next().unwrap(),
			z: it.next().unwrap(),
		};
		Moon { pos, vel }
	}
	pub fn update(&mut self, moons: &Vec<Moon>) {
		for other in moons {
			self.vel.apply_gravity(&self.pos.calc_gravity(&other.pos));
		}
		self.pos.apply_velocity(&self.vel)
	}
	pub fn energy(&self) -> i64 {
		self.vel.energy() * self.pos.energy()
	}
}

fn gcd(a: u64, b: u64) -> u64 {
	if b == 0 {
		a
	} else {
		gcd(b, a % b)
	}
}

fn lcm(x: u64, y: u64) -> u64 {
	x / gcd(x, y) * y
}
fn main() -> Result<(),ureq::Error> {
	let input = BufReader::new(get_input(12)?);
	let moons0: Vec<Moon> = input
		.lines()
		.map(|x| Moon::from_line(&x.unwrap()))
		.collect();
	let mut moons = moons0.clone();
	for _ in 0..1000 {
		let moons_snap = moons.clone();
		for moon in moons.iter_mut() {
			moon.update(&moons_snap);
		}
	}
	let result1: i64 = moons.iter().map(|x| x.energy()).sum();
	let mut moons = moons0.clone();
	let mut i = 0u64;
	let mut x_period = None;
	let mut y_period = None;
	let mut z_period = None;
	let periods = loop {
		i += 1;
		let moons_snap = moons.clone();
		for moon in moons.iter_mut() {
			moon.update(&moons_snap);
		}
		let x_state = |m: &Moon| (m.pos.x, m.vel.x);
		let y_state = |m: &Moon| (m.pos.y, m.vel.y);
		let z_state = |m: &Moon| (m.pos.z, m.vel.z);
		if x_period.is_none() {
			if moons.iter().map(x_state).eq(moons0.iter().map(x_state)) {
				x_period = Some(i);
			}
		};
		if y_period.is_none() {
			if moons.iter().map(y_state).eq(moons0.iter().map(y_state)) {
				y_period = Some(i);
			}
		};
		if z_period.is_none() {
			if moons.iter().map(z_state).eq(moons0.iter().map(z_state)) {
				z_period = Some(i);
			}
		};
		if let (Some(x), Some(y), Some(z)) = (x_period, y_period, z_period) {
			break (x, y, z);
		}
	};
	println!("Result 1: {}", result1);
	println!("Result 2: {:?}", lcm(periods.0, lcm(periods.1, periods.2)));
	Ok(())
}
