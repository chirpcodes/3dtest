// Dependencies

use super::camera::CameraView;
use crate::models::Model;
use crate::controls::ControlState;
use crate::structs::{Vertex, Normal};

use glium::{
	glutin,
	implement_vertex, uniform,
	Frame, Program, Display, Surface,
	VertexBuffer, IndexBuffer,
	index::PrimitiveType,
	texture::SrgbTexture2d
};

use glium::glutin::{
	event_loop::{EventLoop, ControlFlow},
	window::WindowBuilder,
	ContextBuilder,
	event,
	event::{VirtualKeyCode, ElementState}
};

use std::time::{Instant, Duration};

// Shaders

const VERTEX_SHADER_SRC: &'static str = r#"
	#version 150

	in vec3 normal;
	in vec3 position;
	in vec2 tex_coords;

	out vec3 v_normal;
	out vec3 v_position;
	out vec2 v_tex_coords;

	uniform mat4 perspective;
	uniform mat4 view;
	uniform mat4 matrix;

	void main() {
		mat4 modelview = view * matrix;
		
		v_normal = transpose(inverse(mat3(modelview))) * normal;

		gl_Position = perspective * modelview * vec4(position, 1.0);
		v_position = gl_Position.xyz / gl_Position.w;

		v_tex_coords = tex_coords;
	}
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
	#version 150

	in vec3 v_normal;
	in vec3 v_position;
	in vec2 v_tex_coords;

	out vec4 color;

	uniform vec3 u_light;
	uniform sampler2D tex;

	const vec3 ambient_color = vec3(0.5, 0.5, 0.5);
	const vec3 diffuse_color = vec3(0.8, 0.75, 0.75);
	const vec3 specular_color = vec3(1.0, 1.0, 1.0);
	const float gamma = 2.2;

	void main() {
		float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

		vec3 camera_dir = normalize(-v_position);
		vec3 half_dir = normalize(normalize(u_light) + camera_dir);
		float specular = pow(max(dot(half_dir, normalize(v_normal)), 0.0), 16.0);

		vec3 colorLin = vec3(ambient_color + diffuse * diffuse_color + specular * specular_color);
		vec3 colorGamma = pow(colorLin, vec3(1.0 / gamma));
		
		color = vec4(colorGamma, 1.0) * texture(tex, v_tex_coords);
	}
"#;

// Renderer

pub struct Renderer {
	camera: CameraView,
	control: ControlState,
	event_loop: EventLoop<()>,
	display: Display,
	program: Program,

	models: Vec<Model>,

	_last_frame: Instant
}

impl Renderer {
	pub fn new() -> Self {
		// Display Window

		let mut event_loop = EventLoop::new();

		let window = WindowBuilder::new();
		let context = ContextBuilder::new().with_depth_buffer(24);
		let display = Display::new(window, context, &event_loop).unwrap();

		// Program

		let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

		// Create Renderer Instance

		Self {
			camera: CameraView::new(),
			control: ControlState::new(),
			event_loop: event_loop,
			display: display,
			program: program,

			models: vec![],

			_last_frame: Instant::now()
		}
	}

	pub fn run(mut self) {
		let params = glium::DrawParameters {
			depth: glium::Depth {
				test: glium::draw_parameters::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},
			//backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
			.. Default::default()
		};

		self.event_loop.run(move |event, _, control_flow| {
			// Frame Time

			let now = Instant::now();
			
			let delta_time = (now - self._last_frame).as_nanos() as f32 / 1_000_000.0;

			let next_frame_time = now + Duration::from_nanos(16_666_667);
			*control_flow = ControlFlow::WaitUntil(next_frame_time);

			self._last_frame = now;

			// Start draw frame
	
			let mut frame = self.display.draw();
			frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

			let u_light = [0.0, 5.0, 1.0f32];
			let perspective = Self::get_perspective(&frame);
			let view = self.camera.get_view();

			for model in &self.models {
				let uniforms = uniform!{
					u_light: u_light,
					perspective: perspective,
					view: view,
					matrix: [
						[1.0, 0.0, 0.0, 0.0],
						[0.0, 1.0, 0.0, 0.0],
						[0.0, 0.0, 1.0, 0.0],
						[model.position.x, model.position.y, model.position.z, model.scale]
					],
					tex: model.texture.as_ref().unwrap()
				};

				let vertices = model.vert_buf.as_ref().unwrap();
				let normals = model.norm_buf.as_ref().unwrap();
				let indices = model.ix_buf.as_ref().unwrap();
				frame.draw((vertices, normals), indices, &self.program, &uniforms, &params).unwrap();
			}

			frame.finish().unwrap();

			// Event Handler

			match event {
				// Close Event
				event::Event::WindowEvent { event, .. } => match event {
					event::WindowEvent::CloseRequested => {
						*control_flow = ControlFlow::Exit;
						return;
					},
					_ => ()
				},
				// Input Event
				event::Event::DeviceEvent { event, .. } => match event {
					// Keyboard Event
					event::DeviceEvent::Key(input) => match input.state {
						ElementState::Pressed => self.control.key_press(input.virtual_keycode.unwrap()),
						ElementState::Released => self.control.key_release(input.virtual_keycode.unwrap())
					},
					// Button Click
					event::DeviceEvent::Button { button, state, .. } => match state {
						ElementState::Pressed => self.control.mouse_click(button),
						ElementState::Released => self.control.mouse_release(button)
					},
					// Button Move
					event::DeviceEvent::MouseMotion { delta, .. } => if self.control.is_mouse_clicked(3) {
						self.camera.rotate(delta.0, delta.1, &delta_time)
					},
					_ => ()
				},
				_ => ()
			}

			// Camera control

			self.camera.control(&self.control, &delta_time);
		});
	}

	pub fn get_perspective(frame: &Frame) -> [[f32; 4]; 4] {
		let (width, height) = frame.get_dimensions();
		let aspect_ratio = height as f32 / width as f32;

		let fov: f32 = 3.141592 / 3.0;
		let zfar = 1024.0;
		let znear = 0.1;

		let f = 1.0 / (fov / 2.0).tan();

		[
			[f * aspect_ratio, 0.0, 0.0, 0.0],
			[0.0, f, 0.0, 0.0],
			[0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
			[0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0]
		]
	}

	pub fn add_model(&mut self, mut model: Model) {
		if let Some(ref path) = model.tex_path {
			model.load_tex(&self.display);
		} else {
			model.texture = Some(SrgbTexture2d::empty(&self.display, 0, 0).unwrap());
		}

		model.vert_buf = Some(VertexBuffer::new(&self.display, &model.vertices).unwrap());
		model.norm_buf = Some(VertexBuffer::new(&self.display, &model.normals).unwrap());
		model.ix_buf = Some(IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &model.indices).unwrap());
		self.models.push(model);
	}
}