// Dependencies

use image;
use image::ImageFormat;

use crate::structs::{Vertex, Normal, Vec3};
use glium::{
	Display,
	VertexBuffer, IndexBuffer,
	texture::{RawImage2d, SrgbTexture2d}
};

use std::{
	fs::File,
	io::BufReader
};

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
	pub ix_buf: Option<IndexBuffer<u16>>,

	pub tex_path: Option<String>,
	pub texture: Option<SrgbTexture2d>
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
			ix_buf: None,

			tex_path: None,
			texture: None
		}
	}

	pub fn set_tex_path(&mut self, path: String) {
		self.tex_path = Some(path);
	}

	pub fn load_tex(&mut self, display: &Display) {
		let file = File::open(self.tex_path.as_ref().unwrap()).unwrap();
		let read = BufReader::new(file);
		let img = image::load(read, ImageFormat::Png).unwrap().to_rgba8();
		let img_dim = img.dimensions();
		let img_raw = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), img_dim);
		let texture = SrgbTexture2d::new(display, img_raw).unwrap();
		self.texture = Some(texture);
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
			ix_buf: None,

			tex_path: self.tex_path.clone(),
			texture: None
		}
	}
}