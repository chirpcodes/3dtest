use crate::render::{
	lib::view_matrix
};

use crate::structs::Vec3;
use crate::controls::ControlState;

use glium::glutin::event::VirtualKeyCode::*;

// Camera

pub struct CameraView {
	pos: Vec3,
	rotate: Vec3,
	up: Vec3
}

impl CameraView {
	pub fn new() -> Self {
		Self {
			pos: Vec3::new(1.0, 0.0, -0.5),
			rotate: Vec3::new(0.0, 0.0, 1.0),
			up: Vec3::new(0.0, 2.0, 0.0)
		}
	}

	pub fn get_rotation(&self) -> [f32; 3] {
		let x = self.rotate.x.to_radians();
		let y = self.rotate.y.to_radians();
		let z = self.rotate.z.to_radians();

		[
			x.sin() * y.cos(),
			y.sin(),
			z.cos() * y.cos()
		]
	}

	pub fn get_view(&self) -> [[f32; 4]; 4] {

		view_matrix(
			&[self.pos.x, self.pos.y, self.pos.z],
			&self.get_rotation(),
			&[self.up.x, self.up.y, self.up.z]
		)
	}

	pub fn control(&mut self, state: &ControlState, delta: &f32) {
		let mut vel = Vec3::new(0.0, 0.0, 0.0);

		let move_vel = 0.005 * delta;

		let xrot = self.rotate.x.to_radians();
		let yrot = self.rotate.y.to_radians();
		println!("{}", self.rotate.y);

		if state.is_key_pressed(W) { // Forward
			vel.x += move_vel * xrot.sin();
			vel.y += move_vel * yrot.sin();
			vel.z += move_vel * xrot.cos();
		}
		if state.is_key_pressed(A) { // Left
			vel.x -= move_vel * xrot.cos();
			vel.z += move_vel * xrot.sin();
		}
		if state.is_key_pressed(S) { // Back
			vel.x -= move_vel * xrot.sin();
			vel.y -= move_vel * yrot.sin();
			vel.z -= move_vel * xrot.cos();
		}
		if state.is_key_pressed(D) { // Right
			vel.x += move_vel * xrot.cos();
			vel.z -= move_vel * xrot.sin();
		}

		if state.is_key_pressed(Space) {
			vel.y += move_vel;
		}
		if state.is_key_pressed(LShift) {
			vel.y -= move_vel;
		}

		self.pos += vel;
	}

	pub fn rotate(&mut self, x: f64, y: f64, delta: &f32) {
		let rot_delta = 1.0 * delta;

		self.rotate.x = Self::wrap_deg(self.rotate.x + (x as f32) * rot_delta);
		self.rotate.y = Self::wrap_deg(self.rotate.y - (y as f32) * rot_delta);
		self.rotate.z = self.rotate.x;

		//println!("{} = {}", self.rotate.x, self.rotate.x.sin());
	}

	pub fn wrap_deg(mut deg: f32) -> f32 {
		if deg > 180.0 {
			deg = -180.0 + ((deg - 180.0) % 360.0);
		}
		if deg < -180.0 {
			deg = 180.0 - ((180.0 - deg) % 360.0);
		}
		return deg;
	}
}