// Dependencies

use glium::glutin::event::VirtualKeyCode;

// ControlState

pub struct ControlState {
	pressed: Vec<VirtualKeyCode>,
	clicked: Vec<u32>
}

impl ControlState {
	pub fn new() -> Self {
		Self {
			pressed: Vec::<VirtualKeyCode>::new(),
			clicked: Vec::<u32>::new()
		}
	}

	pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
		self.pressed.contains(&key)
	}

	pub fn key_press(&mut self, key: VirtualKeyCode) {
		if !self.is_key_pressed(key) {
			self.pressed.push(key);
		}
	}
	pub fn key_release(&mut self, key: VirtualKeyCode) {
		let pos = self.pressed.iter().position(|x| x == &key);
		if let Some(i) = pos {
			self.pressed.remove(i);
		}
	}
	
	pub fn is_mouse_clicked(&self, button: u32) -> bool {
		self.clicked.contains(&button)
	}
	pub fn mouse_click(&mut self, button: u32) {
		if !self.is_mouse_clicked(button) {
			self.clicked.push(button);
		}
	}
	pub fn mouse_release(&mut self, button: u32) {
		let pos = self.clicked.iter().position(|x| x == &button);
		if let Some(i) = pos {
			self.clicked.remove(i);
		}
	}
}