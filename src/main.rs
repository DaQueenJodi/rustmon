mod mons;
use mons::get_mon_names;
use mons::read_mon;
use std::io;
fn main() {
	let mon_names = get_mon_names();
	println!("choose a mon:");
	for (i, name) in mon_names.clone().into_iter().enumerate() {
		println!("{}: {name}", i + 1);
	}
	let mut index: usize;
	{
		let mut input = String::new();
		loop {
			io::stdin().read_line(&mut input).unwrap();
			index = input.trim().parse().unwrap_or(0);
			if index > 0 && index <= mon_names.len() { break } 
			else { println!("enter a valid number!"); }
		}
	}
	let chosen_name = mon_names[index - 1].clone();
	println!("you chose: {}", chosen_name);
	let chosen_mon = read_mon(chosen_name);
	println!("your mon: {chosen_mon:#?}");
}
