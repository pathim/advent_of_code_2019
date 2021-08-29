use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};

fn distribute_packets(packets: Vec<Int>, buffers: &mut Vec<Vec<Int>>) -> Option<(Int, Int)> {
	let mut nat = None;
	for p in packets.chunks_exact(3) {
		let addr = p[0] as usize;
		if addr == 255 {
			nat = Some((p[1], p[2]));
		} else {
			buffers[addr].push(p[1]);
			buffers[addr].push(p[2]);
		}
	}
	nat
}

fn main() -> Result<(),ureq::Error> {
	let day = 23;
	let input = get_input(day)?;
	let bm = Machine::from_file(input);

	let mut machines = Vec::with_capacity(50);
	let mut packet_buffers = Vec::with_capacity(50);
	for _ in 0..50 {
		packet_buffers.push(Vec::new());
	}
	for i in 0..50 {
		let mut m = bm.clone();
		let (_, r) = m.run(Some(i)).expect("Error");
		if !r.is_empty() {
			println!("Output too soon");
			return Ok(());
		}
		machines.push(m);
	}
	let mut nat = None;
	let mut last0y = 0;
	let mut idle = 0;
	loop {
		for (i, m) in machines.iter_mut().enumerate() {
			let buf = packet_buffers.get_mut(i).expect("missing buffer");
			let mut inp = buf.clone();
			if inp.is_empty() {
				inp.push(-1);
			} else {
				idle = 0;
			}
			buf.clear();
			let (_, r) = m.run(inp).expect("Error");
			if let Some(first) = distribute_packets(r, &mut packet_buffers) {
				//println!("From: {} old: {:?} new:{:?}",i,nat,first);
				if nat.is_none() {
					println!("First solution {}", first.1);
				}
				nat = Some(first);
			}
		}
		if idle > 5 {
			let nat_val = nat.expect("No nat value");
			packet_buffers[0].push(nat_val.0);
			packet_buffers[0].push(nat_val.1);
			if nat_val.1 == last0y {
				println!("Second solution {}", last0y);
				return Ok(());
			}
			last0y = nat_val.1;
		}
		idle += 1;
	}
}
