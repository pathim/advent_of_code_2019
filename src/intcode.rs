use std::convert::TryFrom;
use std::io::BufRead;

pub type Int = i64;
pub trait InOut{
	fn input(&mut self) ->Option<Int>;
	fn output(&mut self, value:Int);
}
pub struct SimpleInOut{
	in_value:Int,
	pub out_values:Vec<Int>,
}
impl SimpleInOut{
	pub fn new(in_value:Int) -> Self{
		Self{in_value,out_values:Vec::new()}
	}
}
impl InOut for SimpleInOut{
    fn input(&mut self) ->Option<Int> {
        Some(self.in_value)
    }

    fn output(&mut self, value:Int) {
        self.out_values.push(value)
    }
}
pub struct DummyInOut;
impl InOut for DummyInOut{
    fn input(&mut self) ->Option<Int> {
        None
    }

    fn output(&mut self, _value:Int) {
    }
}
#[derive(Debug)]
pub enum Error {
	IllegalOp(Int),
	IllegalParamMode(Int),
	NotEnoughInput,
	TriedToWriteImmediate,
}
#[derive(Debug)]
enum ParamMode {
	Pos,
	Imm,
	Rel,
}

impl TryFrom<Int> for ParamMode {
	type Error = Error;

	fn try_from(value: Int) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(ParamMode::Pos),
			1 => Ok(ParamMode::Imm),
			2 => Ok(ParamMode::Rel),
			_ => Err(Error::IllegalParamMode(value)),
		}
	}
}
#[derive(Debug)]
enum Opcode {
	Add(ParamMode, ParamMode, ParamMode),
	Mul(ParamMode, ParamMode, ParamMode),
	Input(ParamMode),
	Output(ParamMode),
	Jit(ParamMode, ParamMode),
	Jif(ParamMode, ParamMode),
	Lt(ParamMode, ParamMode, ParamMode),
	Eq(ParamMode, ParamMode, ParamMode),
	Arb(ParamMode),
	Halt,
}

impl TryFrom<Int> for Opcode {
	type Error = Error;

	fn try_from(value: Int) -> Result<Self, Self::Error> {
		let p1 = ParamMode::try_from((value / 100) % 10)?;
		let p2 = ParamMode::try_from((value / 1000) % 10)?;
		let p3 = ParamMode::try_from((value / 10000) % 10)?;
		match value % 100 {
			1 => Ok(Opcode::Add(p1, p2, p3)),
			2 => Ok(Opcode::Mul(p1, p2, p3)),
			3 => Ok(Opcode::Input(p1)),
			4 => Ok(Opcode::Output(p1)),
			5 => Ok(Opcode::Jit(p1, p2)),
			6 => Ok(Opcode::Jif(p1, p2)),
			7 => Ok(Opcode::Lt(p1, p2, p3)),
			8 => Ok(Opcode::Eq(p1, p2, p3)),
			9 => Ok(Opcode::Arb(p1)),
			99 => Ok(Opcode::Halt),
			x => Err(Error::IllegalOp(x)),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Machine {
	mem: Vec<Int>,
	ip: usize,
	rel_base: Int,
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
		Self::from_vec(mem)
	}
	pub fn from_vec(mem: Vec<Int>) -> Self {
		Self {
			mem,
			ip: 0,
			rel_base: 0,
			running: false,
		}
	}
	pub fn set_mem(&mut self, addr: usize, value: Int) {
		self.mem[addr] = value;
	}
	pub fn get_mem(&self, addr: usize) -> Int {
		self.mem[addr]
	}
	pub fn run<T: InOut>(&mut self,mut inout:T) -> Result<T, Error> {
		self.running = true;
		while self.running {
			let op = Opcode::try_from(self.read_ip_and_advance())?;
			match op {
				Opcode::Add(p1, p2, p3) => self.bin_op(|a, b| a + b, p1, p2, p3)?,
				Opcode::Mul(p1, p2, p3) => self.bin_op(|a, b| a * b, p1, p2, p3)?,
				Opcode::Input(p1) => self.input(p1, &mut inout)?,
				Opcode::Output(p1) => inout.output(self.output(p1)),
				Opcode::Jit(p1, p2) => self.jump_condition(|x| x != 0, p1, p2),
				Opcode::Jif(p1, p2) => self.jump_condition(|x| x == 0, p1, p2),
				Opcode::Lt(p1, p2, p3) => self.bin_op(|a, b| (a < b) as Int, p1, p2, p3)?,
				Opcode::Eq(p1, p2, p3) => self.bin_op(|a, b| (a == b) as Int, p1, p2, p3)?,
				Opcode::Arb(p1) => self.adjust_relative_base(p1),
				Opcode::Halt => self.running = false,
			};
		}
		Ok(inout)
	}

	fn read_ip_and_advance(&mut self) -> Int {
		let val = self.mem[self.ip];
		self.ip += 1;
		val
	}

	fn get_param_value(&self, pm: ParamMode, param: Int) -> Int {
		match pm {
			ParamMode::Pos => *self.mem.get(param as usize).unwrap_or(&0),
			ParamMode::Imm => param,
			ParamMode::Rel => *self.mem.get((self.rel_base + param) as usize).unwrap_or(&0),
		}
	}
	fn set_param_value(&mut self, value: Int, pm: ParamMode, param: Int) -> Result<(), Error> {
		let addr = match pm {
			ParamMode::Pos => param,
			ParamMode::Rel => self.rel_base + param,
			ParamMode::Imm => return Err(Error::TriedToWriteImmediate),
		} as usize;
		if addr >= self.mem.len() {
			self.mem.resize(addr + 1, 0);
		}
		self.mem[addr] = value;
		Ok(())
	}

	fn input(&mut self, pm: ParamMode, input:&mut impl InOut) -> Result<(), Error> {
		let value = input.input().ok_or(Error::NotEnoughInput)?;
		let dest = self.read_ip_and_advance();
		self.set_param_value(value, pm, dest)?;
		Ok(())
	}
	fn output(&mut self, pm1: ParamMode) -> Int {
		let param = self.read_ip_and_advance();
		self.get_param_value(pm1, param)
	}
	fn jump_condition<F: FnOnce(Int) -> bool>(&mut self, cond: F, pm1: ParamMode, pm2: ParamMode) {
		let param1 = self.read_ip_and_advance();
		let param2 = self.read_ip_and_advance();
		let val = self.get_param_value(pm1, param1);
		let dst = self.get_param_value(pm2, param2);
		if cond(val) {
			self.ip = dst as usize;
		}
	}

	fn bin_op<F: FnOnce(Int, Int) -> Int>(
		&mut self,
		op: F,
		pm1: ParamMode,
		pm2: ParamMode,
		pm3: ParamMode,
	) -> Result<(), Error> {
		let param1 = self.read_ip_and_advance();
		let param2 = self.read_ip_and_advance();
		let v1 = self.get_param_value(pm1, param1);
		let v2 = self.get_param_value(pm2, param2);
		let result = op(v1, v2);
		let dest = self.read_ip_and_advance();
		self.set_param_value(result, pm3, dest)
	}
	fn adjust_relative_base(&mut self, pm1: ParamMode) {
		let param = self.read_ip_and_advance();
		let delta = self.get_param_value(pm1, param);
		self.rel_base += delta;
	}
}
