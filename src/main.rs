// Dependencies

mod structs;
mod models;
mod controls;
mod render;

// Main

pub fn main() {
	let renderer = render::Renderer::new();
	renderer.run();
}