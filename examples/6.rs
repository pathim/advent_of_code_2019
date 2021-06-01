use aoc2019::get_input;
use std::{collections::HashMap, io::{BufRead,BufReader}};

fn get_orbit_count(body:&str,orbit_map:&HashMap<String,String>,orbit_count:&mut HashMap<String,u64>)->u64{
	if let Some(res)=orbit_count.get(body){return *res;}
	if let Some(parent)=orbit_map.get(body){
		let v=1+get_orbit_count(parent, orbit_map, orbit_count);
		orbit_count.insert(body.to_owned(), v);
		return v;
	}else{
		return 0;
	}
}

fn get_distance_map(body:&str,orbit_map:&HashMap<String,String>)->HashMap<String,u64>{
	let mut res=HashMap::new();
	let mut current=body;
	let mut distance=0;
	while let Some(parent)=orbit_map.get(current){
		res.insert(parent.to_owned(), distance);
		distance+=1;
		current=parent;
	}
	res
}
fn main() -> std::io::Result<()>{
	let input=BufReader::new(get_input(6)?);
	let mut orbit_map=HashMap::new();
	let mut orbit_count=HashMap::new();
	for l in input.lines(){
		let line=l?;
		let (parent,child)=line.split_once(')').unwrap();
		orbit_map.insert(child.to_owned(), parent.to_owned());
	}
	for body in orbit_map.keys(){
		println!("{}: {}",body,get_orbit_count(body, &orbit_map, &mut orbit_count))
	}
	let result1:u64=orbit_count.values().sum();

	let distances_you=get_distance_map("YOU", &orbit_map);
	let distances_san=get_distance_map("SAN", &orbit_map);
	let mut result2=u64::MAX;
	for (body,dist) in distances_you.iter(){
		if let Some(dist_san)=distances_san.get(body){
			result2=result2.min(dist+dist_san);
		}
	}
	println!("Result 1: {}",result1);
	println!("Result 2: {}",result2);
	Ok(())
}