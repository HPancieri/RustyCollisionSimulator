pub struct Vector {
	x: f32,
	y: f32,
}

impl Vector {
	pub fn new (x: f32, y: f32) -> Vector {
		Vector { x, y }
	}

	pub fn _get_module (&self) -> f32 {
		let m: f32 = self.x.powi(2) + self.y.powi(2);
		m.sqrt()
	}

	pub fn _get_angle (&self) -> f32 {
		self.y.atan2(self.x)
	}

	pub fn add (&mut self, v: &Vector) {
		self.x += v.x;
		self.y += v.y;
	}

	pub fn set_x (&mut self, x: f32) { self.x = x; }
	pub fn set_y (&mut self, y:f32) { self.y = y; }

	pub fn get_x (&self) -> f32 { self.x }
	pub fn get_y (&self) -> f32 { self.y }

}
