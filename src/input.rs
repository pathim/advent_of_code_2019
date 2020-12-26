fn generate_path(day:u32)->std::path::PathBuf{
	["input",&day.to_string()].iter().collect()
}
fn download_input(day:u32)->std::io::Result<()>{
	let cookie=std::fs::read_to_string("cookie")?;
	let resp=ureq::get(&format!("https://adventofcode.com/2019/day/{}/input",day)).set("Cookie",&cookie).call().into_string()?;
	std::fs::write(generate_path(day), resp)?;
	Ok(())
}
pub fn get_input(day:u32)->std::io::Result<std::fs::File>{
	let path=generate_path(day);
	let file_in=std::fs::File::open(&path);
	if file_in.is_ok(){
		file_in
	} else {
		download_input(day)?;
		std::fs::File::open(path)
	}
}