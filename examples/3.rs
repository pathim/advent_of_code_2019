use aoc2019::get_input;
use std::{collections::HashSet, io::{BufRead,BufReader}};

struct Pos{
	x:i64,
	y:i64,
	l:u64,
}

impl Default for Pos{
    fn default() -> Self {
        Self{x:0,y:0,l:0}
    }
}

fn dir_to_locs(cmd:&str,start:Pos,locs:&mut std::collections::HashMap<(i64,i64),u64>)->Pos{
	let dir=cmd.chars().nth(0).unwrap();
	let distance=cmd.trim_end().split_at(1).1.parse::<u64>().unwrap();
	let mut pos=start;
	for _ in 0..distance{
		pos.l+=1;
		match dir {
			'U'=>pos.x-=1,
			'D'=>pos.x+=1,
			'L'=>pos.y-=1,
			'R'=>pos.y+=1,
			_ => panic!("Invalid direction"),
		}
		let k=(pos.x,pos.y);
		if !locs.contains_key(&k){
			locs.insert(k,pos.l);
		}
	}
	pos
}

fn wire_to_locs(wire:&str) -> std::collections::HashMap<(i64,i64),u64>{
	let mut locs=std::collections::HashMap::new();
	let mut pos=Default::default();
	for cmd in wire.split(','){
		pos=dir_to_locs(cmd, pos, &mut locs);
	}
	locs
}
fn main() -> std::io::Result<()>{
	let mut input=BufReader::new(get_input(3)?);
	let mut wire1=Default::default();
	let mut wire2=Default::default();
	input.read_line(&mut wire1)?;
	input.read_line(&mut wire2)?;

	let locs1=wire_to_locs(&wire1);
	let locs1_keys=locs1.keys().map(|x| x.to_owned()).collect::<HashSet<(i64,i64)>>();
	let locs2=wire_to_locs(&wire2);
	let locs2_keys=locs2.keys().map(|x| x.to_owned()).collect::<HashSet<(i64,i64)>>();
	let intersections=locs1_keys.intersection(&locs2_keys);
	let result1=intersections.clone().map(|x| x.0.abs()+x.1.abs()).min().unwrap();
	let result2=intersections.map(|x| locs1[x]+locs2[x]).min().unwrap();
	
	println!("Result 1: {}",result1);
	println!("Result 2: {}",result2);
	Ok(())
}
