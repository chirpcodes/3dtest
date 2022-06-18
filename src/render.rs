mod lib;
mod structs;
mod camera;
mod teapot;

// Dependencies

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

use camera::CameraView;
use crate::controls::ControlState;

// Vertex

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}
implement_vertex!(Vertex, position);

// Renderer

pub struct Renderer {
	camera: CameraView,
	control: ControlState,
	event_loop: EventLoop<()>,
	display: Display,
	program: Program
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
			uniform mat4 matrix;

			void main() {
				v_normal = transpose(inverse(mat3(matrix))) * normal;
				gl_Position = perspective * matrix * vec4(position, 1.0);
			}
		"#;

		let fragment_shader_src = r#"
			#version 150

			in vec3 v_normal;

			out vec4 color;

			uniform vec3 u_light;

			void main() {
				float brightness = dot(normalize(v_normal), normalize(u_light));
				vec3 dark_color = vec3(0.6, 0.6, 0.6);
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
			program: program
		}
	}

	pub fn draw(mut self) {
		self.event_loop.run(move |event, _, control_flow| {
			let next_frame_time = std::time::Instant::now() +
				std::time::Duration::from_nanos(16_666_667);
			*control_flow = ControlFlow::WaitUntil(next_frame_time);

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
						ElementState::Pressed => { self.control.press(input.virtual_keycode.unwrap()) },
						ElementState::Released => { self.control.release(input.virtual_keycode.unwrap()) }
					},
					_ => ()
				},
				_ => ()
			}

			// Start draw frame
	
			let mut frame = self.display.draw();
			frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
	
			let positions = VertexBuffer::new(&self.display, &teapot::VERTICES).unwrap();
			let normals = VertexBuffer::new(&self.display, &teapot::NORMALS).unwrap();
			let indices = IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();
	
			let uniforms = uniform! {
				u_light: [-2.0, 2.0, 1.0f32],
				perspective: Self::get_perspective(&frame),
				view: self.camera.get_view(),
				matrix: [
					[0.01, 0.0, 0.0, 0.0],
					[0.0, 0.01, 0.0, 0.0],
					[0.0, 0.0, 0.01, 0.0],
					[0.0, 0.0, 2.0, 1.0f32]
				]
			};
	
			let params = glium::DrawParameters {
				depth: glium::Depth {
					test: glium::draw_parameters::DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				// backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
				.. Default::default()
			};
	
			frame.draw((&positions, &normals), &indices, &self.program, &uniforms, &params).unwrap();
	
			frame.finish().unwrap();
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
}