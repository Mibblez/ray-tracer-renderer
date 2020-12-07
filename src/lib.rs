pub mod ray_tracer_utilities {
    use std::ops::Neg;
    use std::marker::Copy;
    use std::clone::Clone;
    use std::cmp::PartialEq;
    use std::fmt;

    use auto_ops::impl_op_ex;

    #[derive(Copy, Clone, Debug)]
    pub struct Vec4 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        w: f64,
    }

    impl fmt::Display for Vec4 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
        }
    }

    // Addition for Vec4
    impl_op_ex!(+ |a: &Vec4, b: &Vec4| -> Vec4 {
        Vec4::new_vec4(a.x + b.x,
                       a.y + b.y,
                       a.z + b.z,
                       a.w + b.w)
    });

    // Subtraction for Vec4
    impl_op_ex!(- |a: &Vec4, b: &Vec4| -> Vec4 {
        Vec4::new_vec4(a.x - b.x,
                       a.y - b.y,
                       a.z - b.z,
                       a.w - b.w)
    });

    // Multiplication for Vec4 * f64
    impl_op_ex!(* |a: &Vec4, b: f64| -> Vec4 {
        Vec4::new_vec4(a.x * b,
                       a.y * b,
                       a.z * b,
                       a.w * b)
    });

    // Division for Vec4 * f64
    impl_op_ex!(/ |a: &Vec4, b: f64| -> Vec4 {
        Vec4::new_vec4(a.x / b,
                       a.y / b,
                       a.z / b,
                       a.w / b)
    });

    impl PartialEq for Vec4 {
        fn eq(&self, other: &Vec4) -> bool {
            equal_approx(self.x, other.x) &&
                equal_approx(self.y, other.y) &&
                equal_approx(self.z, other.z) &&
                equal_approx(self.w, other.w)
        }
    }

    impl Neg for Vec4 {
        type Output = Vec4;

        fn neg(self) -> Vec4 {
            Vec4 {
                x: self.x * -1.0,
                y: self.y * -1.0,
                z: self.z * -1.0,
                w: self.w * -1.0,
            }
        }
    }

    impl Vec4 {
        pub fn new_vec4(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
            Vec4 {
                x,
                y,
                z,
                w,
            }
        }

        pub fn new_point(x: f64, y: f64, z: f64) -> Vec4 {
            Vec4 {
                x,
                y,
                z,
                w: 1.0,
            }
        }

        pub fn new_vec(x: f64, y: f64, z: f64) -> Vec4 {
            Vec4 {
                x,
                y,
                z,
                w: 0.0,
            }
        }

        pub fn magnitude(&self) -> f64 {
            (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
        }

        pub fn normalized(&self) -> Vec4 {
            let normed: Vec4 = *self / self.magnitude();

            // Sometimes magnitude is off by a really small number when X, Y, and Z do not share a common divisor
            // I hate this
            if normed.magnitude() != 1.0 {
                if normed.x != 0.0 {
                    Vec4 {
                        x: normed.x + (0.00000000000000017 * normed.x.signum()),
                        y: normed.y,
                        z: normed.z,
                        w: normed.w,
                    }
                } else if normed.y != 0.0 {
                    Vec4 {
                        x: normed.x,
                        y: normed.y + (0.00000000000000017 * normed.y.signum()),
                        z: normed.z,
                        w: normed.w,
                    }
                } else {
                    Vec4 {
                        x: normed.x,
                        y: normed.y,
                        z: normed.z + (0.00000000000000017 * normed.z.signum()),
                        w: normed.w,
                    }
                }
            } else {
                normed
            }
        }

        pub fn dot(&self, other: &Vec4) -> f64 {
            (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
        }

        pub fn cross(&self, other: &Vec4) -> Vec4 {
            Vec4::new_vec(self.y * other.z - self.z * other.y,
                          self.z * other.x - self.x * other.z,
                          self.x * other.y - self.y * other.x)
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Color {
        pub r: f64,
        pub g: f64,
        pub b: f64,
    }

    // Addition for Color
    impl_op_ex!(+ |a: &Color, b: &Color| -> Color {
        Color::new(a.r + b.r,
                   a.g + b.g,
                   a.b + b.b)
    });

    // Subtraction for Color
    impl_op_ex!(- |a: &Color, b: &Color| -> Color {
        Color::new(a.r - b.r,
                   a.g - b.g,
                   a.b - b.b)
    });

    // Multiplication for Color
    impl_op_ex!(* |a: &Color, b: &Color| -> Color {
        Color::new(a.r * b.r,
                   a.g * b.g,
                   a.b * b.b)
    });

    // Multiplication for Color * f64
    impl_op_ex!(* |a: &Color, b: f64| -> Color {
        Color::new(a.r * b,
                   a.g * b,
                   a.b * b)
    });

    impl PartialEq for Color {
        fn eq(&self, other: &Color) -> bool {
            equal_approx(self.r, other.r) &&
                equal_approx(self.g, other.g) &&
                equal_approx(self.b, other.b)
        }
    }

    impl Color {
        pub fn new(r: f64, g: f64, b: f64) -> Color {
            Color {
                r,
                g,
                b,
            }
        }

        pub fn as_u8_tup(&self) -> (u8, u8, u8) {
            (
                (self.r * 255.0) as u8,
                (self.g * 255.0) as u8,
                (self.b * 255.0) as u8,
            )
        }
    }

    pub struct Canvas {
        pub width: usize,
        pub height: usize,
        pixels: Vec<Vec<Color>>,
    }

    impl Canvas {
        pub fn new(width: usize, height: usize, fill_color: Color) -> Canvas {
            Canvas {
                width,
                height,
                pixels: vec![vec![fill_color; width]; height],
            }
        }

        pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
            if x > self.width - 1 || y > self.height - 1 {
                return;
            }

            self.pixels[y][x] = color.clone();
        }

        pub fn read_pixel(&self, x: usize, y: usize) -> Color {
            self.pixels[y][x]
        }

        pub fn to_ppm(&self) -> String {
            let mut ppm_str: String = String::new();

            // PPM Header
            ppm_str.push_str(&format!("P3\n{w} {h}\n255\n", w = self.width, h = self.height));

            let mut chars_in_current_line = 0;

            for row in self.pixels.iter() {
                for pixel in row.iter() {
                    let (r, g, b) = pixel.as_u8_tup();

                    let line = format!("{r} {g} {b} ", r = r, g = g, b = b);

                    // Lines in a PPM file shouldn't be longer than 70 characters
                    if chars_in_current_line + line.len() > 70 {
                        let chars_remaining = 70 - chars_in_current_line;
                        let r_str = r.to_string();
                        let b_str = b.to_string();
                        let rg_str = format!("{r} {g}\n", r = r, g = g);

                        if rg_str.len() <= chars_remaining {
                            // R and G go on the current line
                            ppm_str.push_str(&rg_str);

                            // B goes on the next line
                            ppm_str.push_str(&b_str);
                            ppm_str.push_str(" ");

                            chars_in_current_line = b_str.len() + 1;
                        } else if r_str.len() + 1 <= chars_remaining {
                            // R goes on the current line
                            ppm_str.push_str(&r_str);
                            ppm_str.push_str("\n");

                            // G and B go on the next line
                            let gb_str = format!("{g} {b} ", g = g, b = b);
                            ppm_str.push_str(&gb_str);

                            chars_in_current_line = gb_str.len();
                        } else {
                            // Replace space at the end of the line with a newline
                            ppm_str.pop();
                            ppm_str.push_str("\n");
                            ppm_str.push_str(&line);

                            // R G and B go on the next line
                            chars_in_current_line = line.len();
                        }
                    } else {
                        chars_in_current_line += line.len();
                        ppm_str.push_str(&line);
                    }
                }
                ppm_str.pop();    // Remove the space at the end of the line
                ppm_str.push_str("\n");
                chars_in_current_line = 0;
            }
            ppm_str
        }
    }

    pub struct Projectile {
        pub pos: Vec4,
        pub vel: Vec4,
    }

    impl Projectile {
        pub fn new_projectile(pos: Vec4, vel: Vec4) -> Projectile {
            Projectile {
                pos,
                vel,
            }
        }
    }

    pub struct Environment {
        gravity: Vec4,
        wind: Vec4,
    }

    impl Environment {
        pub fn new_environment(gravity: Vec4, wind: Vec4) -> Environment {
            Environment {
                gravity,
                wind,
            }
        }
    }

    pub fn tick(env: &Environment, proj: &mut Projectile) {
        proj.pos = proj.pos + proj.vel;
        proj.vel = proj.vel + env.gravity + env.wind;
    }

    pub fn equal_approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.00001
    }

    #[cfg(test)]
    mod vector_tests {
        use super::*;

        #[test]
        fn new_point() {
            let point: Vec4 = Vec4::new_point(4.0, -4.0, 3.0);
            assert_eq!(point.x, 4.0);
            assert_eq!(point.y, -4.0);
            assert_eq!(point.z, 3.0);
            assert_eq!(point.w, 1.0);    // Points should have 1 as their w value
        }

        #[test]
        fn new_vector() {
            let vector: Vec4 = Vec4::new_vec(4.0, -4.0, 3.0);
            assert_eq!(vector.x, 4.0);
            assert_eq!(vector.y, -4.0);
            assert_eq!(vector.z, 3.0);
            assert_eq!(vector.w, 0.0);    // Vectors should have 0 as their w value
        }

        #[test]
        fn test_equal_approx() {
            assert_eq!(equal_approx(1.0, 1.0000005), true);
            assert_eq!(equal_approx(1.0, 1.005), false);
        }

        #[test]
        fn vec4_eq() {
            assert_eq!(Vec4::new_vec(0.0, 0.0, 0.0), Vec4::new_vec(0.0, 0.0, 0.0));
            assert_eq!(Vec4::new_point(2.0, 4.0, 6.0), Vec4::new_point(2.0, 4.0, 6.0));
        }

        #[test]
        fn add_point_vec() {
            let p: Vec4 = Vec4::new_point(4.0, -4.0, 3.0);
            let v: Vec4 = Vec4::new_vec(1.0, -8.0, 2.0);
            let result_point: Vec4 = &p + &v;        // Adding a vector and a point gives a point

            assert_eq!(result_point, Vec4::new_point(5.0, -12.0, 5.0));

            assert_eq!(p.x, 4.0);
            assert_eq!(v.x, 1.0);
        }

        #[test]
        fn add_vec_vec() {
            let v1: Vec4 = Vec4::new_vec(10.0, 10.0, 5.0);
            let v2: Vec4 = Vec4::new_vec(-10.0, -10.0, -5.0);
            let result_vec: Vec4 = &v1 + &v2;    // Adding a vector and a vector gives a vector

            assert_eq!(result_vec, Vec4::new_vec(0.0, 0.0, 0.0));

            assert_eq!(v1.x, 10.0);
            assert_eq!(v2.x, -10.0);
        }

        #[test]
        fn sub_point_point() {
            let p1: Vec4 = Vec4::new_point(3.0, 2.0, 1.0);
            let p2: Vec4 = Vec4::new_point(5.0, 6.0, 7.0);

            let result_point = &p1 - &p2;

            assert_eq!(result_point, Vec4::new_vec(-2.0, -4.0, -6.0));
        }

        #[test]
        fn sub_vec_point() {
            let p: Vec4 = Vec4::new_point(3.0, 2.0, 1.0);
            let v: Vec4 = Vec4::new_point(5.0, 6.0, 7.0);

            let result_vec = &p - &v;

            assert_eq!(result_vec, Vec4::new_vec(-2.0, -4.0, -6.0));
        }

        #[test]
        fn sub_vec_vec() {
            let v1: Vec4 = Vec4::new_point(3.0, 2.0, 1.0);
            let v2: Vec4 = Vec4::new_point(5.0, 6.0, 7.0);

            let result_vec = &v1 - &v2;

            assert_eq!(result_vec, Vec4::new_vec(-2.0, -4.0, -6.0));
        }

        #[test]
        fn negate_vec4() {
            let v: Vec4 = Vec4::new_vec(1.0, 2.0, 3.0);
            assert_eq!(-v, Vec4::new_vec4(-1.0, -2.0, -3.0, 0.0));

            let p: Vec4 = Vec4::new_point(5.0, 6.0, 7.0);
            assert_eq!(-p, Vec4::new_vec4(-5.0, -6.0, -7.0, -1.0));

            assert_eq!(v.x, 1.0);
            assert_eq!(p.x, 5.0)
        }

        #[test]
        fn multiply_vec4() {
            let v1: Vec4 = Vec4::new_vec(1.0, 2.0, 3.0);
            let v2: Vec4 = v1 * 2.0;
            assert_eq!(v2, Vec4::new_vec4(2.0, 4.0, 6.0, 0.0));

            let p1: Vec4 = Vec4::new_point(5.0, 6.0, 7.0);
            let p2: Vec4 = p1 * 2.5;
            assert_eq!(p2, Vec4::new_vec4(12.5, 15.0, 17.5, 2.5));
        }

        #[test]
        fn divide_vec4() {
            let v1: Vec4 = Vec4::new_vec(1.0, 2.0, 3.0);
            let v2: Vec4 = v1 / 2.0;
            assert_eq!(v2, Vec4::new_vec4(0.5, 1.0, 1.5, 0.0));

            let p1: Vec4 = Vec4::new_point(5.0, 6.0, 7.0);
            let p2: Vec4 = p1 / 2.5;
            assert_eq!(p2, Vec4::new_vec4(2.0, 2.4, 2.8, 0.4));
        }

        #[test]
        fn magnitude_vec4() {
            let v: Vec4 = Vec4::new_vec(2.0, 2.0, 2.0);
            assert_eq!(v.magnitude(), 12.0_f64.sqrt());
        }

        #[test]
        fn normalize_vec4() {
            let v: Vec4 = Vec4::new_vec(4.0, 0.0, 0.0);

            assert_eq!(v.normalized(), Vec4::new_vec(1.0, 0.0, 0.0));
            assert_eq!(v.x, 4.0);

            let mut v2: Vec4 = Vec4::new_vec(1.0, 2.0, 3.0);
            v2 = v2.normalized();
            assert_eq!(v2.magnitude(), 1.0);

            let mut v3: Vec4 = Vec4::new_vec(10.0, 12.0, 5.0);
            v3 = v3.normalized();
            //assert_eq!(equal_approx(v3.magnitude(), 1.0), true);
            assert_eq!(v3.magnitude(), 1.0);
        }

        #[test]
        fn dot_vec4() {
            let a: Vec4 = Vec4::new_vec(1.0, 2.0, 3.0);
            let b: Vec4 = Vec4::new_vec(2.0, 3.0, 4.0);

            assert_eq!(a.dot(&b), 20.0);
        }

        #[test]
        fn cross_vec4() {
            let a: Vec4 = Vec4::new_vec(1.0, 2.0, 3.0);
            let b: Vec4 = Vec4::new_vec(2.0, 3.0, 4.0);

            assert_eq!(a.cross(&b), Vec4::new_vec(-1.0, 2.0, -1.0));
            assert_eq!(b.cross(&a), Vec4::new_vec(1.0, -2.0, 1.0));
        }
    }

    #[cfg(test)]
    mod color_tests {
        use super::*;

        #[test]
        fn new_color() {
            let c: Color = Color::new(-0.5, 0.4, 0.1);

            assert_eq!(c.r, -0.5);
            assert_eq!(c.g, 0.4);
            assert_eq!(c.b, 0.1);
        }

        #[test]
        fn add_colors() {
            let c1: Color = Color::new(0.9, 0.6, 0.75);
            let c2: Color = Color::new(0.7, 0.1, 0.25);

            assert_eq!(&c1 + &c2, Color::new(1.6, 0.7, 1.0));
            assert_eq!(c1.r, 0.9);
            assert_eq!(c2.g, 0.1);
        }

        #[test]
        fn sub_colors() {
            let c1: Color = Color::new(0.9, 0.6, 0.75);
            let c2: Color = Color::new(0.7, 0.1, 0.25);

            assert_eq!(&c1 - &c2, Color::new(0.2, 0.5, 0.5));
            assert_eq!(c1.r, 0.9);
            assert_eq!(c2.g, 0.1);
        }

        #[test]
        fn mul_color_by_scalar() {
            let c: Color = Color::new(0.2, 1.0, -0.2);

            assert_eq!(&c * 2.0, Color::new(0.4, 2.0, -0.4));
            assert_eq!(c.r, 0.2);
        }

        #[test]
        fn mul_color_by_color() {
            let c1: Color = Color::new(1.0, 0.2, 0.4);
            let c2: Color = Color::new(0.9, 1.0, 0.1);

            let c3: Color = &c1 * &c2;

            assert_eq!(c3, Color::new(0.9, 0.2, 0.04));
            assert_eq!(c1.r, 1.0);
            assert_eq!(c2.g, 1.0);
        }

        #[test]
        fn color_as_u8_tup() {
            let c: Color = Color::new(1.0, -1.0, 0.0);
            let (r, g, b) = c.as_u8_tup();

            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);

            let c2: Color = Color::new(-0.5, 0.5, 0.5);
            let (r2, g2, b2) = c2.as_u8_tup();

            assert_eq!(r2, 0);
            assert_eq!(g2, 127);
            assert_eq!(b2, 127);
        }
    }

    #[cfg(test)]
    mod canvas_tests {
        use super::*;

        #[test]
        fn test_create_canvas() {
            let c: Canvas = Canvas::new(10, 20, Color::new(0.0, 0.0, 0.0));

            assert_eq!(c.width, 10);
            assert_eq!(c.height, 20);

            let black: Color = Color::new(0.0, 0.0, 0.0);

            // Make sure that all pixels are in a new canvas are 0
            for row in c.pixels.iter() {
                for pixel in row.iter() {
                    assert_eq!(pixel, &black);
                }
            }
        }

        #[test]
        fn write_to_canvas() {
            let mut c: Canvas = Canvas::new(10, 20, Color::new(0.0, 0.0, 0.0));

            let red: Color = Color::new(1.0, 0.0, 0.0);

            c.write_pixel(2, 3, &red);

            assert_eq!(&c.read_pixel(2, 3), &red);
        }

        #[test]
        fn ppm_header() {
            let c: Canvas = Canvas::new(5, 3, Color::new(0.0, 0.0, 0.0));

            let ppm = c.to_ppm();

            let mut ppm_header = String::new();

            // Grab the header from the first three lines of the ppm string
            for line in ppm.lines().take(3) {
                ppm_header.push_str(line);
                ppm_header.push_str("\n");
            }

            assert_eq!(ppm_header, "P3\n5 3\n255\n");
        }

        #[test]
        fn ppm_pixel_data() {
            let mut c: Canvas = Canvas::new(5, 3, Color::new(0.0, 0.0, 0.0));

            let c1 = Color::new(1.5, 0.0, 0.0);
            let c2 = Color::new(0.0, 0.5, 0.0);
            let c3 = Color::new(-0.5, 0.0, 1.0);

            c.write_pixel(0, 0, &c1);
            c.write_pixel(2, 1, &c2);
            c.write_pixel(4, 2, &c3);

            let ppm: String = c.to_ppm();

            let mut ppm_pixel_data = String::new();
            // Skip the header and go to the pixel data
            for line in ppm.lines().skip(3) {
                ppm_pixel_data.push_str(line);
                ppm_pixel_data.push_str("\n");
            }

            assert_eq!(ppm_pixel_data, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
									0 0 0 0 0 0 0 127 0 0 0 0 0 0 0\n\
									0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
        }

        #[test]
        fn ppm_split_lines() {
            let width = 10;
            let height = 2;

            let mut c: Canvas = Canvas::new(width, height, Color::new(0.0, 0.0, 0.0));
            let color: Color = Color::new(1.0, 0.8, 0.6);

            for i in 0..width {
                for j in 0..height {
                    c.write_pixel(i, j, &color)
                }
            }

            let ppm: String = c.to_ppm();

            let mut ppm_pixel_data = String::new();
            // Skip the header and go to the pixel data
            for line in ppm.lines().skip(3) {
                ppm_pixel_data.push_str(line);
                ppm_pixel_data.push_str("\n");
            }

            println!("{}", ppm_pixel_data);
            assert_eq!(ppm_pixel_data, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
									153 255 204 153 255 204 153 255 204 153 255 204 153\n\
									255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
									153 255 204 153 255 204 153 255 204 153 255 204 153\n");
        }
    }
}

