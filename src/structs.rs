use glium::implement_vertex;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}
implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}
implement_vertex!(Normal, normal);

pub struct Vec2 {
	pub x: f32,
	pub y: f32
}
impl Vec2 {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x:x, y:y }
	}
}
impl Add for Vec2 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}
impl AddAssign for Vec2 {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
	}
}

#[derive(Debug)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}
impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x:x, y:y, z:z }
	}
}
impl Add for Vec3 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z
		}
	}
}
impl AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}

pub struct Vec4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}
impl Vec4 {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self { x:x, y:y, z:z, w:w }
	}
}
impl Add for Vec4 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
			w: self.w + other.w
		}
	}
}
impl AddAssign for Vec4 {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
		self.w += other.w;
	}
}