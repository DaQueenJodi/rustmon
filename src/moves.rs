pub struct Move {
	name: String,
	description: String,
	self_effects: Vec<mon::Effect>,
	enemy_effects: Vec<mon::Effect>,
	damage: i32, // if negative, heal
	self_damage: i32,
}

impl Default for Move {
	
}
