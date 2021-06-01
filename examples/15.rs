use std::collections::HashMap;

use aoc2019::get_input;
use aoc2019::intcode::Machine;
use itertools::Itertools;

#[derive(Debug,PartialEq,Clone,Copy)]
enum Tile{
	Wall,Empty(u32),Goal(u32),Start
}

fn map_all(machine:Machine,pos:(i32,i32),map:&mut HashMap<(i32,i32),Tile>,dist:u32){
	let new_pos=[(pos.0,pos.1-1),(pos.0,pos.1+1),(pos.0-1,pos.1),(pos.0+1,pos.1)];
	for (dir,p) in (1..=4).zip(new_pos){
		if !map.contains_key(&p){
			let mut m=machine.clone();
			let result=m.run(Some(dir)).unwrap().1.first().unwrap().to_owned();
			let new_tile=match result{
				0 => Tile::Wall,
				1 => Tile::Empty(dist+1),
				2 => Tile::Goal(dist+1),
				_ => panic!("Invalid response")
			};
			map.insert(p, new_tile);
			if matches!(new_tile,Tile::Empty(_)){
				map_all(m, p, map,dist+1);
			}
		}
	}
}

fn map_oxy(pos:(i32,i32),oxy_map:&mut HashMap<(i32,i32),Tile>,dist:u32){
	let new_pos=[(pos.0,pos.1-1),(pos.0,pos.1+1),(pos.0-1,pos.1),(pos.0+1,pos.1)];
	for p in new_pos{
		if let Tile::Empty(d)=oxy_map[&p]{
			if dist+1<d{
				oxy_map.insert(p,Tile::Empty(dist+1));
				map_oxy(p, oxy_map, dist+1)
			}
		}
	}
}

fn main() -> std::io::Result<()> {
	let day = 15;
	println!("Day {}", day);
	let base_machine = Machine::from_file(get_input(day)?);
	let m = base_machine.clone();
	let mut map=HashMap::new();
	let pos=(0,0);
	map.insert(pos, Tile::Start);
	map_all(m, pos, &mut map,0);
	let (min_x,max_x)=map.keys().map(|v|v.0).minmax().into_option().unwrap();
	let (min_y,max_y)=map.keys().map(|v|v.1).minmax().into_option().unwrap();
	for tile in map.values(){
		if let Tile::Goal(n)=tile{
			println!("Result 1: {}",n);
		}
	}
	let mut goal_pos=Default::default();
	for (k,x) in map.iter_mut(){
		match x{
			Tile::Empty(_) => *x=Tile::Empty(u32::MAX),
			Tile::Goal(_) => {*x=Tile::Goal(0);goal_pos=*k},
			_=>{}
		}
	}
	map_oxy(goal_pos, &mut map, 0);
	let result2=map.values().map(|x| if let Tile::Empty(d)=x{*d}else{0}).max().unwrap();
	println!("Result 2: {}",result2);
	for y in min_y..=max_y{
		for x in min_x..=max_x{
			match *map.get(&(x,y)).unwrap_or(&Tile::Wall){
				Tile::Empty(_) => print!(" "),
				Tile::Wall => print!("#"),
				Tile::Goal(_) => print!("X"),
				Tile::Start => print!("S"),
			};
			
		}
		println!("");
	}



	Ok(())
}
