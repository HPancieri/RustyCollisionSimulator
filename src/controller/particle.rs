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

	pub fn set_velocity (&mut self, vx: f32, vy: f32) {
		self.velocity.set_x(vx);
		self.velocity.set_y(vy);
	}

	pub fn set_position (&mut self, px: f32, py: f32) {
		self.position.set_x(px);
		self.position.set_x(py);
	}

	pub fn invert_x_velocity (&mut self) { self.velocity.set_x(-self.velocity.get_x()); }

	pub fn invert_y_velocity (&mut self) { self.velocity.set_y(-self.velocity.get_y()); }

	pub fn draw (&self, graphics: &mut Graphics2D) {
		// Drawing the particle
		graphics.draw_circle((self.position.get_x(), self.position.get_y()), self.radius, self.color);
	}

	pub fn get_x_pos (&self) -> f32 { self.position.get_x() }
	pub fn get_y_pos (&self) -> f32 { self.position.get_y() }
	pub fn _get_x_vel (&self) -> f32 { self.velocity.get_x() }
	pub fn _get_y_vel (&self) -> f32 { self.velocity.get_y() }
	pub fn get_r (&self) -> f32 { self.radius }
	pub fn _get_mass (&self) -> f32 { self.mass }


	pub fn get_distance_between (p1: &Particle, p2: &Particle) -> f32 {
		let d: f32 = (p1.position.get_x() - p2.position.get_x()).powi(2) + (p1.position.get_y() - p2.position.get_y()).powi(2);
		d.sqrt()
	}
 

	pub fn check_collision (p1: &Particle, p2: &Particle) -> bool {
		Particle::get_distance_between(p1, p2) <= p1.radius + p2.radius
	}


	pub fn collide (particles: &mut Vec<Particle>, i: usize, j: usize) {
		// Getting particle 1 information
		let m1: f32 = particles[i].mass;
		let v1x: f32 = particles[i].velocity.get_x();
		let v1y: f32 = particles[i].velocity.get_y();

		//  Getting particle 2 information
		let m2: f32 = particles[j].mass;
		let v2x: f32 = particles[j].velocity.get_x();
		let v2y: f32 = particles[j].velocity.get_y();

		// Calculating final velocity for particle 1
		let v1xf: f32 = (m1 - m2)/(m1 + m2) * v1x + (2.0*m2)/(m1 + m2) * v2x;
		let v1yf: f32 = (m1 - m2)/(m1 + m2) * v1y + (2.0*m2)/(m1 + m2) * v2y;

		// Calculating final velocity for particle 2
		let v2xf: f32 = (m2 - m1)/(m1 + m2) * v2x + (2.0*m1)/(m1 + m2) * v1x;
		let v2yf: f32 = (m2 - m1)/(m1 + m2) * v2y + (2.0*m1)/(m1 + m2) * v1y;

		// Setting new velocity values for x and y directions
		particles[i].set_velocity(v1xf, v1yf);
		particles[j].set_velocity(v2xf, v2yf);
	}
}
