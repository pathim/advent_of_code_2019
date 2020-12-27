use std::convert::TryFrom;
use std::io::BufRead;

type Int = i32;
#[derive(Debug)]
pub enum Error {
	IllegalOp(Int),
	IllegalParamMode(Int),
	NotEnoughInput,
}
#[derive(Debug)]
enum ParamMode {
	Pos,
	Imm,
}

impl ParamMode {
	pub fn get_value(&self, param: Int, mem: &[Int]) -> Int {
		match self {
			ParamMode::Pos => mem[param as usize],
			ParamMode::Imm => param,
		}
	}
}

impl TryFrom<Int> for ParamMode {
	type Error = Error;

	fn try_from(value: Int) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(ParamMode::Pos),
			1 => Ok(ParamMode::Imm),
			_ => Err(Error::IllegalParamMode(value)),
		}
	}
}
#[derive(Debug)]
enum Opcode {
	Add(ParamMode, ParamMode),
	Mul(ParamMode, ParamMode),
	Input,
	Output(ParamMode),
	Jit(ParamMode, ParamMode),
	Jif(ParamMode, ParamMode),
	Lt(ParamMode, ParamMode),
	Eq(ParamMode, ParamMode),
	Halt,
}

impl TryFrom<Int> for Opcode {
	type Error = Error;

	fn try_from(value: Int) -> Result<Self, Self::Error> {
		let p1 = ParamMode::try_from((value / 100) % 10)?;
		let p2 = ParamMode::try_from((value / 1000) % 10)?;
		let _p3 = ParamMode::try_from((value / 10000) % 10)?;
		match value % 100 {
			1 => Ok(Opcode::Add(p1, p2)),
			2 => Ok(Opcode::Mul(p1, p2)),
			3 => Ok(Opcode::Input),
			4 => Ok(Opcode::Output(p1)),
			5 => Ok(Opcode::Jit(p1, p2)),
			6 => Ok(Opcode::Jif(p1, p2)),
			7 => Ok(Opcode::Lt(p1, p2)),
			8 => Ok(Opcode::Eq(p1, p2)),
			99 => Ok(Opcode::Halt),
			x => Err(Error::IllegalOp(x)),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Machine {
	mem: Vec<Int>,
	ip: usize,
	running: bool,
}

impl Machine {
	pub fn from_file(file: std::fs::File) -> Self {
		let reader = std::io::BufReader::new(file);
		let mem = reader
			.split(b',')
			.filter_map(|x| x.ok())
			.filter_map(|x| String::from_utf8_lossy(&x).trim().parse().ok())
			.collect();
		Self {
			mem,
			ip: 0,
			running: false,
		}
	}
	pub fn from_vec(mem: Vec<Int>) -> Self {
		Self {
			mem,
			ip: 0,
			running: false,
		}
	}
	pub fn set_mem(&mut self, addr: usize, value: Int) {
		self.mem[addr] = value;
	}
	pub fn get_mem(&self, addr: usize) -> Int {
		self.mem[addr]
	}
	pub fn run<T: IntoIterator<Item = Int>>(&mut self, input: T) -> Result<Vec<Int>, Error> {
		self.running = true;
		let mut input_iter = input.into_iter();
		let mut output = Vec::<Int>::new();
		while self.running {
			let op = Opcode::try_from(self.read_ip_and_advance())?;
			match op {
				Opcode::Add(p1, p2) => self.add(p1, p2),
				Opcode::Mul(p1, p2) => self.mul(p1, p2),
				Opcode::Input => self.input(&mut input_iter)?,
				Opcode::Output(p1) => output.push(self.output(p1)),
				Opcode::Jit(p1, p2) => self.jump_if_true(p1, p2),
				Opcode::Jif(p1, p2) => self.jump_if_false(p1, p2),
				Opcode::Lt(p1, p2) => self.less_than(p1, p2),
				Opcode::Eq(p1, p2) => self.equals(p1, p2),
				Opcode::Halt => self.running = false,
			};
		}
		Ok(output)
	}

	fn read_ip_and_advance(&mut self) -> Int {
		let val = self.mem[self.ip];
		self.ip += 1;
		val
	}

	fn add(&mut self, pm1: ParamMode, pm2: ParamMode) {
		let v1 = pm1.get_value(self.read_ip_and_advance(), &self.mem);
		let v2 = pm2.get_value(self.read_ip_and_advance(), &self.mem);
		let dest = self.read_ip_and_advance();
		self.mem[dest as usize] = v1 + v2;
	}
	fn mul(&mut self, pm1: ParamMode, pm2: ParamMode) {
		let v1 = pm1.get_value(self.read_ip_and_advance(), &self.mem);
		let v2 = pm2.get_value(self.read_ip_and_advance(), &self.mem);
		let dest = self.read_ip_and_advance();
		self.mem[dest as usize] = v1 * v2;
	}
	fn input(&mut self, input: &mut dyn Iterator<Item = Int>) -> Result<(), Error> {
		let value = input.next().ok_or(Error::NotEnoughInput)?;
		let dest = self.read_ip_and_advance();
		self.mem[dest as usize] = value;
		Ok(())
	}
	fn output(&mut self, pm1: ParamMode) -> Int {
		pm1.get_value(self.read_ip_and_advance(), &self.mem)
	}
	fn jump_if_true(&mut self, pm1: ParamMode, pm2: ParamMode) {
		let val = pm1.get_value(self.read_ip_and_advance(), &self.mem);
		let dst = pm2.get_value(self.read_ip_and_advance(), &self.mem);
		if val != 0 {
			self.ip = dst as usize;
		}
	}
	fn jump_if_false(&mut self, pm1: ParamMode, pm2: ParamMode) {
		let val = pm1.get_value(self.read_ip_and_advance(), &self.mem);
		let dst = pm2.get_value(self.read_ip_and_advance(), &self.mem);
		if val == 0 {
			self.ip = dst as usize;
		}
	}

	fn less_than(&mut self, pm1: ParamMode, pm2: ParamMode) {
		let v1 = pm1.get_value(self.read_ip_and_advance(), &self.mem);
		let v2 = pm2.get_value(self.read_ip_and_advance(), &self.mem);
		let dest = self.read_ip_and_advance();
		self.mem[dest as usize] = (v1 < v2) as i32;
	}

	fn equals(&mut self, pm1: ParamMode, pm2: ParamMode) {
		let v1 = pm1.get_value(self.read_ip_and_advance(), &self.mem);
		let v2 = pm2.get_value(self.read_ip_and_advance(), &self.mem);
		let dest = self.read_ip_and_advance();
		self.mem[dest as usize] = (v1 == v2) as i32;
	}
}
