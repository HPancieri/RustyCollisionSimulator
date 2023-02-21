use crate::controller::vector::Vector;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use speedy2d::Graphics2D;
use speedy2d::color::Color;

pub struct Particle {
	mass: f32,
	radius: f32,
	position: Vector,
	velocity: Vector,
	color: Color,
}

impl Particle {
	pub fn new (x: f32, y: f32) -> Particle {
		let mut rng = thread_rng();
		let mass = rng.gen_range(1.0..10.0);
		let possible_colors: [u32; 9] = [0x005F73, 0x0A9396, 0x94D2BD, 0xE9D8A6, 0xEE9B00, 0xCA6702, 0xBB3E03, 0xAE2012, 0x9B2226];
		let color = possible_colors.choose(&mut rng).unwrap();

		Particle {
			mass,
			radius: mass * 6.0,
			position: Vector::new(x, y),
			velocity: Vector::new(rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0)),
			color: Color::from_hex_rgb(*color),
		}
	}

	pub fn update (&mut self) {
		self.position.add(&self.velocity);
	}

	pub fn set_velocity (&mut self, new_velocity: f32, new_direction: f32) {
		self.velocity.set_x(new_velocity * new_direction.cos());
		self.velocity.set_y(new_velocity * new_direction.sin());
	}

	pub fn invert_x_velocity (&mut self) { self.velocity.set_x(-self.velocity.get_x()); }

	pub fn invert_y_velocity (&mut self) { self.velocity.set_y(-self.velocity.get_y()); }

	pub fn draw (&self, graphics: &mut Graphics2D) {
		// Drawing the particle
		graphics.draw_circle((self.position.get_x(), self.position.get_y()), self.radius, self.color);
	}

	pub fn get_x_pos (&self) -> f32 { self.position.get_x() }
	pub fn get_y_pos (&self) -> f32 { self.position.get_y() }

	pub fn get_r (&self) -> f32 { self.radius }
}
