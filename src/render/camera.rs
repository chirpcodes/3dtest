use crate::render::{
	structs::Vec3,
	lib::view_matrix
};

// Camera

pub struct CameraView {
	pos: Vec3,
	yaw: Vec3,
	rotate: Vec3
}

impl CameraView {
	pub fn new() -> Self {
		Self {
			pos: Vec3 {x:0.0, y:0.0, z:0.0},
			rotate: Vec3 {x:0.0, y:1.0, z:0.0},
			yaw: Vec3 {x:0.0, y:0.0, z:1.0},
		}
	}

	pub fn get_view(&self) -> [[f32; 4]; 4] {
		view_matrix(
			&[self.pos.x, self.pos.y, self.pos.z],
			&[self.rotate.x, self.rotate.y, self.rotate.z],
			&[self.yaw.x, self.yaw.z, self.yaw.z]
		)
	}
}