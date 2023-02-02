use serde_derive::{Deserialize};
use std::path::Path;
use std::vec::Vec;
use std::fs;

#[derive(Deserialize)]
struct MonJson {
	max_health: u32,
	max_pp: u32
}
#[derive(Debug)]
pub struct Mon {
	name: String,
	max_health: u32,
	health: u32,
	max_pp: u32,
	pp: u32
}
fn mon_from_json(mon_json: &MonJson, name: String) -> Mon {
	Mon {
		max_health: mon_json.max_health,
		max_pp: mon_json.max_pp,
		name: name,
		pp: mon_json.max_pp,
		health: mon_json.max_health,
	}
}

pub fn read_mon(name: String) -> Mon {
	let file = fs::File::open("mons/".to_string() + &name + ".json").unwrap();
	let mon_json: MonJson =  serde_json::from_reader(file).unwrap();
	mon_from_json(&mon_json, name)
}
pub fn get_mon_names() -> Vec<String> {
	fs::read_dir(Path::new("mons")).unwrap()
		.into_iter()
		.map(|f| f.unwrap()
				 .file_name()
				 .to_str()
				 .unwrap()
				 .strip_suffix(".json")
				 .unwrap()
				 .to_string())
		.collect()
}

