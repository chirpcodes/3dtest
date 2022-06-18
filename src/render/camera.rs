use crate::render::{
	lib::view_matrix
};

use crate::structs::Vec3;
use crate::controls::ControlState;

use glium::glutin::event::VirtualKeyCode::*;

// Camera

pub struct CameraView {
	pos: Vec3,
	yaw: Vec3,
	rotate: Vec3
}

impl CameraView {
	pub fn new() -> Self {
		Self {
			pos: Vec3::new(1.0, 0.0, -0.5),
			rotate: Vec3::new(0.0, 0.0, 1.0),
			yaw: Vec3::new(0.0, 2.0, 0.0)
		}
	}

	pub fn get_view(&self) -> [[f32; 4]; 4] {
		view_matrix(
			&[self.pos.x, self.pos.y, self.pos.z],
			&[self.rotate.x, self.rotate.y, self.rotate.z],
			&[self.yaw.x, self.yaw.y, self.yaw.z]
		)
	}

	pub fn control(&mut self, state: &ControlState, delta: &f32) {
		let mut vel = Vec3::new(0.0, 0.0, 0.0);

		let move_vel = 0.005 * delta;

		if state.is_key_pressed(W) {
			vel.z += move_vel;	
		}
		if state.is_key_pressed(A) {
			vel.x -= move_vel;
		}
		if state.is_key_pressed(S) {
			vel.z -= move_vel;
		}
		if state.is_key_pressed(D) {
			vel.x += move_vel;
		}

		if state.is_key_pressed(Space) {
			vel.y += move_vel;
		}
		if state.is_key_pressed(LShift) {
			vel.y -= move_vel;
		}

		self.pos += vel;
	}
}