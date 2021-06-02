use std::{collections::HashMap, io::{BufRead, BufReader}};

use aoc2019::get_input;
#[derive(Debug)]
struct Reaction{
	target:String,
	amount:u64,
	sources:HashMap<String,u64>,
}

impl Reaction{
	pub fn from_line(line:&str) -> Self{
		let (s,t)=line.split_once("=>").unwrap();
		let (amount,target)=Self::amount_name_from_string(t);
		let mut sources=HashMap::new();
		for s in s.split(','){
			let (a,n)=Self::amount_name_from_string(s);
			sources.insert(n, a);

		}
		Self{target,amount,sources}
	}
	fn amount_name_from_string(s:&str)->(u64,String){
		let (a,n)=s.trim().split_once(' ').unwrap();
		(a.parse().unwrap(),n.to_owned())
	}
}

fn get_needed(name:&str,amount:u64,reactions:&HashMap<String,Reaction>,waste:&mut HashMap<String,u64>)->u64{
	if name=="ORE"{return amount;}
	let r=reactions.get(name).unwrap();
	let already_available=*waste.get(name).unwrap_or(&0);
	let to_produce=amount.saturating_sub(already_available);
	let actual_amount=if to_produce%r.amount!=0{to_produce-(to_produce%r.amount)+r.amount}else{to_produce};
	let leftover=actual_amount+already_available-amount;
	waste.insert(name.to_owned(), leftover);
	let multiple=actual_amount/r.amount;
	r.sources.iter().map(|(sn,sa)| get_needed(&sn, sa*multiple, reactions,waste)).sum()
}
fn main() -> std::io::Result<()>{
	let input=BufReader::new(get_input(14)?);
	let mut reactions=HashMap::new();
	for l in input.lines(){
		let r=Reaction::from_line(&l?);
		reactions.insert(r.target.clone(), r);
	}
	let mut waste=HashMap::new();
	let mut result=get_needed("FUEL", 1, &reactions,&mut waste);
	let mut fuel=1;
	let mut nmin=u64::MAX;
	println!("Result 1: {}",result);
	for _ in 0..50000 {
		let needed=get_needed("FUEL", 1, &reactions,&mut waste);
		result+=needed;
		if needed<nmin{
			println!("Needed {} after {}",needed,fuel);
			nmin=needed;
		}
		fuel+=1;
	};
	println!("Result 2: {} fuel for {} ore",fuel,result);

	Ok(())
}