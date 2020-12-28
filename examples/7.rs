use aoc2019::get_input;
use aoc2019::intcode::{InOut,Machine,Int};
use itertools::Itertools;

struct Amp{
	data: Vec<Int>,
	output: Int,
}

impl Amp{
	fn new(phase:Int, input:Int) -> Self{
		let mut data=Vec::with_capacity(2);
		data.push(input);
		data.push(phase);
		Amp{data,output:0}
	}
	fn get_out(&self) -> Int {
		self.output
	}
}

impl InOut for Amp{
    fn input(&mut self) ->Option<Int> {
        self.data.pop()
    }

    fn output(&mut self, value:Int) {
        self.output=value;
    }
}

fn main() -> std::io::Result<()> {
	let day=7;
	println!("Day {}",day);
	let base_machine=Machine::from_file(get_input(day)?);
	let phases=0..=4;
	let mut max_out=0;
	for mut p in phases.permutations(5){
		let mut input=0;
		let out=loop{
			let mut m=base_machine.clone();
			if let Some(phase)=p.pop(){
				let a=Amp::new(phase, input);
				let res=m.run(a).expect("Error during execution");
				input=res.get_out();
			}else{
				break input;
			}
		};
		if out>max_out{
			max_out=out;
		}
	}
	println!("First solution {}",max_out);

	Ok(())
}