mod vector;
mod particle;

use particle::Particle;

use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;

use rand::{self, Rng};


pub fn run(number_of_particles: i32) {
	let window = Window::new_centered("Rusty Collision Simulator", (1200, 800)).unwrap();
	let mut rng = rand::thread_rng();

	let mut win = MyWindowHandler {
		particles: Vec::new(),
	};

	// Positioning particles
	for _ in 0..number_of_particles {
		let mut p: Particle = Particle::new(rng.gen_range(60.0..1140.0), rng.gen_range(60.0..740.0));

		for i in 0..win.particles.len() {
			while Particle::check_collision(&win.particles[i], &p) {
				p.set_position(rng.gen_range(60.0..1140.0), rng.gen_range(60.0..740.0));
			}
		}

		win.particles.push(p);
	}

	window.run_loop(win);
}


struct MyWindowHandler {
	particles: Vec<Particle>,
}

impl WindowHandler for MyWindowHandler {
	fn on_draw(
			&mut self,
			helper: &mut WindowHelper<()>,
			graphics: &mut Graphics2D
		) {
		// Clear the screen
		graphics.clear_screen(Color::from_hex_rgb(0x002B3C));

		// Update and draw all particles
		for i in 0..self.particles.len() {
			self.particles[i].update();
			self.particles[i].draw(graphics);
		}

		// Checking for collisions against the walls
		for particle in self.particles.iter_mut() {
			if particle.get_x_pos() - particle.get_r() < 0.0 || particle.get_x_pos() + particle.get_r() > 1200.0 {
				// The particle is out of bounds in the x direction
				particle.invert_x_velocity();
			}
			if particle.get_y_pos() - particle.get_r() < 0.0 || particle.get_y_pos() + particle.get_r() > 800.0 {
				// The particle is out of bounds in the y direction
				particle.invert_y_velocity();
			}
		}

		// Checking for collisions amongst the particles themselves
		for i in 0..self.particles.len() {
			for j in i+1..self.particles.len() {
				if Particle::check_collision(&self.particles[i], &self.particles[j]) {
					Particle::collide(&mut self.particles, i, j);
				}
			}
		}

		// Draw the frame
		helper.request_redraw();
	}
}

