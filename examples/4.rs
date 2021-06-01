use aoc2019::get_input;
use std::io::Read;

fn check_rules(value:u64)->bool{
	let s=format!("{}",value);
	let value=s.as_bytes();

	if value.len()!=6{return false;}
	if !value.iter().skip(1).zip(value.iter()).any(|(a,b)| a==b){return false}
	if value.iter().skip(1).zip(value.iter()).any(|(a,b)| a<b){return false}
	true
}
fn check_rules2(value:u64)->bool{
	let s=format!("{}",value);
	let value=s.as_bytes();

	if value.len()!=6{return false;}
	if value.iter().skip(1).zip(value.iter()).any(|(a,b)| a<b){return false}
	let mut lastval=0;
	let mut runs=Vec::new();
	let mut run_len=0;
	for v in value{
		if *v==lastval{
			run_len+=1;
		}else{
			runs.push(run_len);
			run_len=1;
		}
		lastval=*v;
	}
	runs.push(run_len);
	runs.into_iter().find(|&x|x==2).is_some()
}

fn main() -> std::io::Result<()>{
	let mut input=get_input(4)?;
	let mut buf=String::new();
	input.read_to_string(&mut buf)?;
	let (start,end)=buf.trim_end().split_once('-').unwrap();
	let start:u64=start.parse().unwrap();
	let end:u64=end.parse().unwrap();
	let mut result1=0;
	let mut result2=0;
	for i in start..=end{
		if check_rules(i){
			result1+=1;
		}
		if check_rules2(i){
			result2+=1;
		}
	}
	println!("Result 1: {}",result1);
	println!("Result 2: {}",result2);
	Ok(())
}