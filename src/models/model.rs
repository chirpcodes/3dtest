// Dependencies

use crate::structs::{Vertex, Normal, Vec3};
use glium::{VertexBuffer, IndexBuffer};

// Model

#[derive(Debug)]
pub struct Model {
	pub position: Vec3,
	pub scale: f32,

	pub vertices: Vec<Vertex>,
	pub normals: Vec<Normal>,
	pub indices: Vec<u16>,

	pub vert_buf: Option<VertexBuffer<Vertex>>,
	pub norm_buf: Option<VertexBuffer<Normal>>,
	pub ix_buf: Option<IndexBuffer<u16>>
}

impl Model {
	pub fn new() -> Self {
		Self {
			position: Vec3::new(0.0, 0.0, 0.0),
			scale: 1.0,

			vertices: vec![],
			normals: vec![],
			indices: vec![],

			vert_buf: None,
			norm_buf: None,
			ix_buf: None
		}
	}
}

impl Clone for Model {
	fn clone(&self) -> Self {
		Self {
			position: self.position.clone(),
			scale: self.scale,

			vertices: self.vertices.clone(),
			normals: self.normals.clone(),
			indices: self.indices.clone(),

			vert_buf: None,
			norm_buf: None,
			ix_buf: None
		}
	}
}