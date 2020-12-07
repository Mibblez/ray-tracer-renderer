mod lib;

pub use crate::lib::ray_tracer_utilities::*;

use std::io::Write;

fn main() {
	let start = Vec4::new_point(0.0, 1.0, 0.0);
	let velocity = Vec4::new_vec(1.0, 1.8, 0.0).normalized() * 11.25;
	let mut proj = Projectile::new_projectile(start, velocity);

	let gravity = Vec4::new_vec(0.0, -0.1, 0.0);
	let wind = Vec4::new_vec(-0.01, 0.0, 0.0);
	let env = Environment::new_environment(gravity, wind);

	let mut c = Canvas::new(900, 550, Color::new(0.0, 0.0, 0.0));

	let red = Color::new(1.0,0.0,0.0);

	for _ in 1..250 {
		let x = proj.pos.x.round() as usize;
		let y = (c.height as i64 - (proj.pos.y.round() as i64)) as usize;

		c.write_pixel(x, y, &red);
		tick(&env, &mut proj)
	}

	let ppm = c.to_ppm();

	let mut file = std::fs::File::create("test_image.ppm").expect("create failed");
	file.write_all(ppm.as_bytes()).expect("write failed");

}














