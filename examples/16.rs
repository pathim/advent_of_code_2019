use aoc2019::get_input;
use std::io::{BufRead, BufReader};
struct RepeatElements<T:Iterator+Sized>{
	count:usize,
	current:usize,
	value:Option<T::Item>,
	iter:T
}

impl<T:Iterator> Iterator for RepeatElements<T> where T::Item:Clone+std::fmt::Debug{
    type Item=T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current==0{
			self.value=self.iter.next();
		}
		self.current+=1;
		if self.current==self.count{
			self.current=0;
		}
		self.value.clone()
    }
}

trait RepIter:Iterator+Sized+Clone {
	fn repeat_elements(self,count:usize)->RepeatElements<Self>;	
}

impl <I:Iterator+Sized+Clone> RepIter for I{
    fn repeat_elements(self,count:usize)->RepeatElements<Self> {
        RepeatElements{count,current:0,value:None,iter:self}
    }
}

fn fft_phase(data:&[i32]) -> Vec<i32>{
	let pattern=[0,1,0,-1];
	let mut res=Vec::from(data);
	for (i,v) in res.iter_mut().enumerate(){
		let pat=pattern.iter().cycle().repeat_elements(i+1).skip(1);
		*v=data.iter().zip(pat).skip(i).map(|(a,b)| a*b).sum::<i32>().abs()%10;
	}
	res	
}
fn main() -> std::io::Result<()> {
	let mut input=BufReader::new(get_input(16)?);
	let mut buf=String::new();
	input.read_line(&mut buf)?;
	let offset:usize=buf[0..7].parse().unwrap();
	let mut data:Vec<i32>=buf.trim().as_bytes().iter().map(|x|(x-b'0') as i32).collect();
	let mut data2=data.repeat(10000);
	for i in 0..100{
		println!("{}",i);
		data=fft_phase(&data);
		//data2=fft_phase(&data2);

	}
	let result1=&data[..8];
	let result2=&data2[offset..offset+8];
	print!("result1: ");
	for d in result1{
		print!("{}",d);
	}
	println!();
	print!("result2: ");
	for d in result2{
		print!("{}",d);
	}
	println!();
	Ok(())

}