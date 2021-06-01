use aoc2019::get_input;
use std::{collections::HashSet, f64::consts::PI, io::{BufRead, BufReader}};

fn count_asteroids(pos:(usize,usize),asteroids:&Vec<(usize,usize)>)->usize{
	asteroids.iter().map(|(x,y)| (((*y as f64)-(pos.1 as f64)).atan2((*x as f64)-(pos.0 as f64))*1000000.0) as i64).collect::<HashSet<i64>>().len()
}

fn wrap_angle(angle:f64) -> f64{
	if angle.is_sign_negative(){
		angle+2.0*PI
	} else {
		angle
	}
}
fn sorted_asteroids(pos:(usize,usize),asteroids:&Vec<(usize,usize)>)->Vec<((f64,f64),(usize,usize))>{
	let mut angle_dist=asteroids.iter().map(|(x,y)| ((*x as f64)-(pos.0 as f64),(*y as f64)-(pos.1 as f64))).map(|(x,y)| (wrap_angle((x).atan2(-y)),x*x+y*y)).zip(asteroids.iter().map(|&x|(x.0,x.1))).collect::<Vec<((f64,f64),(usize,usize))>>();
	angle_dist.sort_by(|x,y| x.0.0.partial_cmp(&y.0.0).unwrap().then(x.0.1.partial_cmp(&y.0.1).unwrap()));
	let mut last_angle=100.0;
	let mut angle_adjust=2.0*PI;
	for ((ad,_),_) in angle_dist.iter_mut(){
		let angle=*ad;
		if angle==last_angle{
			*ad+=angle_adjust;
			angle_adjust+=2.0*PI;
		} else {
			angle_adjust=2.0*PI;
		}
		last_angle=angle;
	}
	angle_dist.sort_by(|a,b|a.0.0.partial_cmp(&b.0.0).unwrap());
	angle_dist//.into_iter().map(|(_,y)| y).collect()
}

fn main() -> std::io::Result<()>{
	let input=BufReader::new(get_input(10)?);
	let asteroids:Vec<(usize,usize)>=input.lines().enumerate().map(|(y,line)| line.unwrap().chars().enumerate().filter_map(|(x,c)| if c=='#' {Some((x,y))}else{None}).collect::<Vec<(usize,usize)>>()).flatten().collect();

	let best_asteroid=asteroids.iter().max_by_key(|&x| count_asteroids(*x, &asteroids)).unwrap();
	let result1=count_asteroids(*best_asteroid, &asteroids);

	let sorted=sorted_asteroids(*best_asteroid, &asteroids);
	let result2=sorted[200-1];

	println!("Result 1: {}",result1);
	println!("Result 2: {}",result2.1.0*100+result2.1.1);
	Ok(())
}