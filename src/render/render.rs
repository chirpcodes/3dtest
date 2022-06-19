// Dependencies

use super::camera::CameraView;
use crate::controls::ControlState;
use crate::models::Model;
use crate::structs::Vertex;

use super::teapot;

use glium::{
	glutin,
	implement_vertex, uniform,
	Frame, Program, Display, Surface,
	VertexBuffer, IndexBuffer,
	index::PrimitiveType
};

use glium::glutin::{
	event_loop::{EventLoop, ControlFlow},
	window::WindowBuilder,
	ContextBuilder,
	event,
	event::{VirtualKeyCode, ElementState}
};

use std::time::Instant;

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

		// Shaders

		let vertex_shader_src = r#"
			#version 150

			in vec3 position;
			in vec3 normal;

			out vec3 v_normal;

			uniform mat4 perspective;
			uniform mat4 view;
			uniform mat4 matrix;

			void main() {
				mat4 modelview = view * matrix;
				v_normal = transpose(inverse(mat3(modelview))) * normal;
				gl_Position = perspective * modelview * vec4(position, 1.0);
			}
		"#;

		let fragment_shader_src = r#"
			#version 150

			in vec3 v_normal;

			out vec4 color;

			uniform vec3 u_light;

			void main() {
				float brightness = dot(normalize(v_normal), normalize(u_light));
				vec3 dark_color = vec3(0.505, 0.5, 0.5);
				vec3 regular_color = vec3(1.0, 1.0, 1.0);
				color = vec4(mix(dark_color, regular_color, brightness), 1.0);
			}
		"#;

		// Program

		let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

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
		self.event_loop.run(move |event, _, control_flow| {
			// Frame Time

			let start = Instant::now();
			let delta_time = (start - self._last_frame).as_nanos() as f32 / 1_000_000.0;
			self._last_frame = start;

			// Start draw frame
	
			let mut frame = self.display.draw();
			frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

			/*let positions = VertexBuffer::new(&self.display, &teapot::VERTICES).unwrap();
			let normals = VertexBuffer::new(&self.display, &teapot::NORMALS).unwrap();
			let indices = IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();*/

			let u_light = [-2.0, 2.0, 1.0f32];
			let perspective = Self::get_perspective(&frame);
			let view = self.camera.get_view();

			let params = glium::DrawParameters {
				depth: glium::Depth {
					test: glium::draw_parameters::DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				//backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
				.. Default::default()
			};

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
					]
				};

				let vertices = VertexBuffer::new(&self.display, &model.vertices).unwrap();
				let normals = VertexBuffer::new(&self.display, &model.normals).unwrap();
				let indices = IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &model.indices).unwrap();

				frame.draw((&vertices, &normals), &indices, &self.program, &uniforms, &params).unwrap();
			}

			frame.finish().unwrap();
	
			/*let uniforms = uniform! {
				u_light: [-2.0, 2.0, 1.0f32],
				perspective: Self::get_perspective(&frame),
				view: self.camera.get_view(),
				matrix: [
					[0.01, 0.0, 0.0, 0.0],
					[0.0, 0.01, 0.0, 0.0],
					[0.0, 0.0, 0.01, 0.0],
					[0.0, 0.0, 0.0, 1.0f32]
				]
			};
	
			let params = glium::DrawParameters {
				depth: glium::Depth {
					test: glium::draw_parameters::DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				//backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
				.. Default::default()
			};
	
			frame.draw((&positions, &normals), &indices, &self.program, &uniforms, &params).unwrap();
	
			frame.finish().unwrap();*/

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

			// Next frame time

			let end = Instant::now();
			let next_frame_time = end +
				std::time::Duration::from_nanos(16_666_667);
			*control_flow = ControlFlow::WaitUntil(next_frame_time);
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

	pub fn add_model(&mut self, model: Model) {
		self.models.push(model);
	}
}