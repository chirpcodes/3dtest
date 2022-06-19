// Dependencies

use crate::structs::{Vertex, Normal, Vec3};

// Model

#[derive(Debug)]
pub struct Model {
	pub position: Vec3,

	pub scale: f32,

	pub vertices: Vec<Vertex>,
	pub normals: Vec<Normal>,
	pub indices: Vec<u16>
}

impl Model {
	pub fn new() -> Self {
		Self {
			position: Vec3::new(0.0, 0.0, 0.0),

			scale: 1.0,

			vertices: vec![],
			normals: vec![],
			indices: vec![]
		}
	}
}