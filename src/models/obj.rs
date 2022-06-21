// Dependencies

use std::char;
use std::io::Read;

use crate::models::Model;
use crate::structs::{Vertex, Normal};

// ObjModel

pub struct ObjModel {}

impl ObjModel {
	pub fn parse<T: Read>(file: &mut T) -> Model {
		let mut model = Model::new();
		
		model.vertices.push(
			Vertex { position: (0.0, 0.0, 0.0) }
		);
		model.normals.push(
			Normal { normal: (0.0, 0.0, 0.0) }
		);

		let mut cmd = "".to_owned();
		let mut token = "".to_owned();
		let mut args: Vec<String> = vec![];
		let mut comment = false;
		let mut ignore = false;

		let mut buf = [0u8];
		while let Ok(ct) = file.read(&mut buf) {
			if ct == 0 {
				break;
			}

			let ch = buf[0] as char;
			if comment || ignore {
				if ch == '\n' {
					comment = false;
					ignore = false;
				} else if ignore && ch == ' ' {
					ignore = false;
				} else {
					continue;
				}
			}

			match ch {
				'\n' => {
					if cmd != "" {
						args.push(token.to_owned());
						
						match cmd.as_str() {
							"v" => {
								model.vertices.push(
									Vertex { position: (
										args[0].parse().unwrap(),
										args[1].parse().unwrap(),
										args[2].parse().unwrap()
									) }
								);
							},
							"vn" => {
								let normal = Normal { normal: (
									args[0].parse().unwrap(),
									args[1].parse().unwrap(),
									args[2].parse().unwrap()
								) };

								model.normals.push(
									normal
								);
							},
							"f" => {
								for i in &args {
									model.indices.push(
										i.parse().unwrap()
									);
								}
							},
							"s" => {
								model.scale = args[0].parse().unwrap();
							},
							_ => println!("'{cmd}' not supported")
						}
					}

					cmd.clear();
					token.clear();
					args.clear();
				},
				' ' => {
					let tk = token.to_owned();
					if cmd == "" {
						cmd = tk;
					} else {
						args.push(tk);
					}
					token.clear();
				}
				'#' => {
					comment = true;
				},
				'/' => {
					ignore = true;
				},
				'\r' => {
					continue;
				},
				_ => {
					token.push(ch);
				}
			}
		}

		if model.normals.len() == 0 {
			for v in &model.vertices {
				model.normals.push(
					Normal { normal: (
						v.position.0.cos(),
						v.position.1.sin(),
						v.position.2.sin()
					) }
				);
			}
		}

		model
	}
}