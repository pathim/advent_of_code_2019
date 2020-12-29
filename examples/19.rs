use std::cmp::max;

use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};

fn is_beam(mut m: Machine, x: Int, y: Int) -> bool {
	let i = Vec::from([x, y]);
	let (_, res) = m.run(i).expect("Execution error");
	*res.first().expect("No output") == 1
}

fn find_edge(m: &Machine, y: Int, xstart: Int, xend: Int) -> (Int, Int) {
	let mut x0 = 0;
	for x in xstart.. {
		if is_beam(m.clone(), x, y) {
			x0 = x;
			break;
		}
	}
	let mut x1 = 0;
	for x in max(x0, xend).. {
		if !is_beam(m.clone(), x, y) {
			x1 = x;
			break;
		}
	}
	(x0, x1)
}

fn main() -> std::io::Result<()> {
	let day = 19;
	let input = get_input(day)?;
	let bm = Machine::from_file(input);
	let mut first = 0;
	for y in 0..50 {
		for x in 0..50 {
			let val = is_beam(bm.clone(), x, y);
			first += val as i32;
		}
	}
	println!("First solution {}", first);

	let mut xstart = 0;
	let mut xend = 0;
	let mut second = 0;
	for y in 10.. {
		// skip beginning. algorithm does not work with lines without beam
		let (x0, x1) = find_edge(&bm, y, xstart, xend);
		xstart = x0;
		xend = x1;
		if xend - xstart < 100 {
			continue;
		}
		let (x0_1, _) = find_edge(&bm, y + 99, xstart, xend);
		if xend - x0_1 == 100 {
			second = x0_1 * 10000 + y;
			break;
		}
	}
	println!("Second solution {}", second);
	/*
	Idea for faster solution:
	 - The edges of the beam are linear
	 - Sample some lines to find the edges
	 - do linear fit to find y_0(x)=a*x and y_1(x)=b*x
	 - solve y_0(x)-100=y_1(x+100)=> a*x-100=b*x+100*b => x=(100*b+100)/(a-b)
	 - solution 10000*x+y_0(x)-100 => (10000+a)*x-100
	 (All formulas modulo off-by-one-errors)
	 */

	Ok(())
}
