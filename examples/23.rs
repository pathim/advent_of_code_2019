use aoc2019::get_input;
use aoc2019::intcode::{Int, Machine};

fn distribute_packets(packets: Vec<Int>, buffers: &mut Vec<Vec<Int>>) -> Option<Int> {
	for p in packets.chunks_exact(3) {
		let addr = p[0] as usize;
		if addr == 255 {
			return Some(p[2]);
		}
		buffers[addr].push(p[1]);
		buffers[addr].push(p[2]);
	}
	None
}

fn main() -> std::io::Result<()> {
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
		if let Some(first) = distribute_packets(r, &mut packet_buffers) {
			println!("First solution {}", first);
			return Ok(());
		}
		machines.push(m);
	}
	println!("loop");
	loop {
		for (i, m) in machines.iter_mut().enumerate() {
			let buf = packet_buffers.get_mut(i).expect("missing buffer");
			let mut inp = buf.clone();
			if inp.is_empty() {
				inp.push(-1);
			}
			buf.clear();
			let (_, r) = m.run(inp).expect("Error");
			if let Some(first) = distribute_packets(r, &mut packet_buffers) {
				println!("First solution {}", first);
				return Ok(());
			}
		}
	}

	Ok(())
}
