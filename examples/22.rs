#![feature(toowned_clone_into)]
use std::{io::{BufRead, BufReader}, str::FromStr};
use aoc2019::get_input;

#[derive(Debug)]
enum Command{
	DealNew,
	Cut(i64),
	DealIncr(u128),
}

impl FromStr for Command {
    type Err=std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.starts_with("deal into"){return Ok(Self::DealNew);}
		if line.starts_with("cut"){
			let (_,cut_index)=line.split_once(' ').unwrap();
			return Ok(Self::Cut(cut_index.trim().parse()?));
		}
		let (_,incr)=line.rsplit_once(' ').unwrap();
		Ok(Self::DealIncr(incr.trim().parse()?))
    }
}

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = (base * (base % modulus))%modulus;
    }
    result
}

#[derive(Debug)]
struct LinModExpr{
	modulus:u128,
	a:u128,
	b:u128,
}

impl LinModExpr{
	fn new(modulus:u128) -> Self{
		Self{modulus,a:1,b:0}
	}

	fn add_command(&mut self, command:&Command){
		match command {
			Command::DealNew => {
				self.a=(self.modulus-self.a).rem_euclid(self.modulus);
				let bn=self.modulus-self.b;
				self.b=if bn==self.modulus {self.modulus-1} else {bn-1};
			}
			Command::Cut(n) => {
				self.b=((self.b as i128)-(*n as i128)).rem_euclid(self.modulus as i128) as u128;
			}
			Command::DealIncr(n) => {
				self.a=(n*self.a).rem_euclid(self.modulus);
				self.b=(n*self.b).rem_euclid(self.modulus);
			}
			
		}
	}

	fn inverse(&self)->Self{
		// Assume modulus is prime
		let a=mod_pow(self.a, self.modulus-2, self.modulus);
		let b=((self.modulus-self.b)*a).rem_euclid(self.modulus);
		Self{a,b,modulus:self.modulus}
	}

	fn repeat(&self,n:u128) ->Self{
		// new_a=a^n; new_b=b*(sum_i=0^n-1 a^i)
		let a=mod_pow(self.a, n, self.modulus);
		let b=((a-1)*(mod_pow(self.a-1, self.modulus-2, self.modulus)*self.b).rem_euclid(self.modulus)).rem_euclid(self.modulus);
		Self{a,b,modulus:self.modulus}
	}

	fn eval(&self, value:u128) ->u128{
		((self.a*value).rem_euclid(self.modulus)+self.b).rem_euclid(self.modulus)
	}
}

fn main() -> std::io::Result<()>{
	let input=BufReader::new(get_input(22)?);
	let commands:Vec<Command>=input.lines().map(|x| x.unwrap().parse().unwrap()).collect();

	let mut lin_expr=LinModExpr::new(10007);
	let mut lin_expr2=LinModExpr::new(119315717514047);
	for c in commands.iter(){
		lin_expr.add_command(c);
		lin_expr2.add_command(c)
	}
	println!("Result 1: {}",lin_expr.eval(2019));

	println!("Result 2: {}",lin_expr2.inverse().repeat(101741582076661).eval(2020));

	Ok(())
}