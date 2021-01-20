mod lib;

pub use crate::lib::ray_tracer_utilities::*;
pub use crate::lib::matrices::*;
pub use crate::lib::rays::*;

use std::io::Write;
use crate::get_intersection;

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

fn circle_outline() {
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

fn draw_sphere_isometric() {
	let mut c = Canvas::new(200, 200, Color::new(0.0, 0.0, 0.0));
	let red = Color::new(255.0, 0.0, 0.0);

	let mut s = Sphere::new_sphere(0);

	s.set_transform(Mat4::id().translate(75.0, 75.0, 0.0).scale(8.0, 8.0, 1.0));

	for i in 0..c.width {
		for j in 0..c.height {
			let r = Ray::new_ray(Vec4::new_point(i as f64, j as f64, 0.0),
			Vec4::new_vec(0.0, 0.0, 1.0));

			let xs = get_intersection(&s, &r);
			if xs.len() != 0 {
				c.write_pixel(i, j, &red);
			}
		}
	}

	let ppm = c.to_ppm();

	let mut file = std::fs::File::create("sphere_iso.ppm").expect("create failed");
	file.write_all(ppm.as_bytes()).expect("write failed");
}

fn draw_sphere_perspective() {
	let canvas_size = 100;
	let mut c = Canvas::new(canvas_size, canvas_size,
							Color::new(0.0, 0.0, 0.0));
	let red = Color::new(255.0, 0.0, 0.0);

	let mut s = Sphere::new_sphere(0);

	// Start the ray behind the sphere
	let ray_origin = Vec4::new_point(0.0, 0.0, -10.0);

	let wall_z = 10.0;		// Wall's Z distance from the origin
	let wall_size = 7.0;	// X and Y size of the wall. The entire wall will be rendered
	let half_wall_size = wall_size / 2.0;

	// Size of a pixel in world units
	let pixel_size = wall_size / canvas_size as f64;

	for i in 0..c.width {
		// Translate Y pixels to world units
		let world_y = half_wall_size - pixel_size * i as f64;

		for j in 0..c.height {
			// Translate X pixels to world units
			let world_x = -half_wall_size + pixel_size * j as f64;

			// The point on the wall the ray will hit
			let position = Vec4::new_point(world_x, world_y, wall_z);

			// Cast a ray from the origin to that point
			let r = Ray::new_ray(ray_origin, (position - ray_origin).normalized());

			let xs = get_intersection(&s, &r);
			if xs.len() != 0 {
				c.write_pixel(i, j, &red);
			}
		}
	}

	let ppm = c.to_ppm();

	let mut file = std::fs::File::create("sphere_perspective.ppm").expect("create failed");
	file.write_all(ppm.as_bytes()).expect("write failed");
}


fn main() {
	//projectile_arc();
	circle_outline();
	draw_sphere_isometric();
	draw_sphere_perspective();
}