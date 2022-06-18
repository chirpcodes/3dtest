// Dependencies

use glium::glutin::event::VirtualKeyCode;

// ControlState

pub struct ControlState {
	pressed: Vec<VirtualKeyCode>
}

impl ControlState {
	pub fn new() -> Self {
		Self {
			pressed: Vec::<VirtualKeyCode>::new()
		}
	}

	pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
		self.pressed.contains(&key)
	}

	pub fn press(&mut self, key: VirtualKeyCode) {
		if !self.is_key_pressed(key) {
			self.pressed.push(key);
		}
	}
	pub fn release(&mut self, key: VirtualKeyCode) {
		let pos = self.pressed.iter().position(|x| x == &key);
		if let Some(i) = pos {
			self.pressed.remove(i);
		}
	}
}