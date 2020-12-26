use std::convert::TryFrom;
#[derive(Debug)]
pub enum Error{
	IllegalOp(i32)
}
#[derive(Debug)]
enum Opcode{
	Add,
	Mul,
	Halt,
}

impl TryFrom<i32> for Opcode{
	type Error=Error;
	
    fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value{
			1 => Ok(Opcode::Add),
			2 => Ok(Opcode::Mul),
			99 => Ok(Opcode::Halt),
			x => Err(Error::IllegalOp(x))
			
		}
    }
}

impl Opcode{
	fn size(&self) -> usize{
		match self {
			Opcode::Add|Opcode::Mul=>4,
			Opcode::Halt=>1
		}
	}
}

#[derive(Debug)]
struct Op{
	opcode:Opcode,
	params:[usize;3],
}

impl Op{
	pub fn read(mem:&[i32]) -> Result<Op,Error>{
		let opcode=Opcode::try_from(mem[0])?;
		let params=[mem[1] as usize,mem[2] as usize,mem[3] as usize];
		Ok(Op{opcode,params})
	}
	pub fn exec(&self,mem:&mut [i32],ip:&mut usize) -> bool{
		*ip+=self.opcode.size();
		match self.opcode{
			Opcode::Add=>{
				mem[self.params[2]]=mem[self.params[0]]+mem[self.params[1]];
				true
			},
			Opcode::Mul=>{
				mem[self.params[2]]=mem[self.params[0]]*mem[self.params[1]];
				true
			},
			Opcode::Halt=>false,
		}
	}
}

pub fn init_mem(data:&str)->Vec<i32>{
	data.split(",").filter_map(|x|x.parse().ok()).collect()
}


pub fn execute_one(mem:&mut Vec<i32>,ip:&mut usize) -> Result<bool,Error>{
	let op=Op::read(&mem[*ip..])?;
	Ok(op.exec(mem, ip))
}

pub fn execute(mem:&mut Vec<i32>) -> Result<(),Error>{
	let mut ip=0;
	while execute_one(mem, &mut ip)?{
	};
	Ok(())
}