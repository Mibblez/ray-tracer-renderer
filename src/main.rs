mod lib;

pub use crate::lib::ray_tracer_utilities::*;
pub use crate::lib::matrices::*;

use std::io::Write;

fn projectile_arc() {
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

	let mut file = std::fs::File::create("projectile_arc.ppm").expect("create failed");
	file.write_all(ppm.as_bytes()).expect("write failed");
}

fn draw_circle() {
	use std::f64::consts::PI;

	let mut c = Canvas::new(400, 400, Color::new(0.0, 0.0, 0.0));
	let white = Color::new(255.0, 255.0, 255.0);

	let origin = Vec4::new_point(0.0, 0.0, 0.0);
	let rotation = Mat4::new_rotation_z(PI / 6.0);

	let mut p = Mat4::new_translation(150.0, 0.0, 0.0) * origin;

	for _ in 0..12 {
		p = rotation * p;

		c.write_pixel((p.x + 200.0) as usize, (p.y + 200.0) as usize, &white);
	}

	let ppm = c.to_ppm();

	let mut file = std::fs::File::create("circle.ppm").expect("create failed");
	file.write_all(ppm.as_bytes()).expect("write failed");
}

fn main() {
	projectile_arc();
	draw_circle();
}














