// Exports

mod structs;
mod models;
mod controls;
mod render;

// Dependencies

use std::fs::File;

// Main

pub fn main() {
	let mut file = File::open("./data/teapot.obj").unwrap();
	let model = models::ObjModel::parse(&mut file);

	let mut renderer = render::Renderer::new();
	renderer.add_model(model);
	renderer.run();
}