// Exports

mod structs;
mod models;
mod controls;
mod render;

// Dependencies

use std::fs::File;

// Main

pub fn main() {
	let mut file = File::open("./data/m0766b0001_0.obj").unwrap();
	let mut file2 = File::open("./data/m0766b0001_1.obj").unwrap();
	
	/*let mut model = models::ObjModel::parse(&mut file);
	model.set_tex_path("./data/v01_m8375b0001_d.png".to_string());*/

	let mut model = models::ObjModel::parse(&mut file);
	model.set_tex_path("./data/v01_m0766b0001_d.png".to_string());

	let mut model2 = models::ObjModel::parse(&mut file2);
	model2.set_tex_path("./data/v01_m0766b0001_d.png".to_string());

	let mut renderer = render::Renderer::new();

	renderer.add_model(model);
	renderer.add_model(model2);

	renderer.run();
}