// Dependencies

use std::char;
use std::io::Read;

use crate::models::Model;
use crate::structs::{Vertex, Normal};

// ObjModel

pub struct ObjModel {}

impl ObjModel {
	pub fn parse<T: Read>(file: &mut T) {
		let mut model = Model::new();

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

			let mut process = false;
			let mut eol = false;
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
								model.normals.push(
									Normal { normal: (
										args[0].parse().unwrap(),
										args[1].parse().unwrap(),
										args[2].parse().unwrap()
									) }
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
				_ => {
					token.push(ch);
				}
			}

			//println!("{:?}", buf[0] as char == '\n');
		}
	}
}