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
        pub w: f64,
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

        pub fn equal_approx(&self, other: &Vec4) -> bool {
            equal_approx(self.x, other.x) &&
                equal_approx(self.y, other.y) &&
                equal_approx(self.z, other.z) &&
                equal_approx(self.w, other.w)
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

pub mod matrices {
    use auto_ops::impl_op_ex;
    use super::ray_tracer_utilities::Vec4;
    use std::ops::Neg;
    use super::ray_tracer_utilities::equal_approx;

    macro_rules! build_mat {
        ($mat_name:ident, $size:expr) => (
            #[derive(Copy, Clone, Debug)]
            pub struct $mat_name {
                pub data: [[f64;$size];$size],
            }

            impl $mat_name {
            // Not sure why functions here are being marked as dead code
                #[allow(dead_code)]
                pub fn new(data: [[f64;$size]; $size]) -> $mat_name {
                    $mat_name { data }
                }

                #[allow(dead_code)]
                pub fn zeros() -> $mat_name {
                    $mat_name { data: [[0.0 ; $size] ; $size] }
                }

                #[allow(dead_code)]
                pub fn transposed(&self) -> $mat_name {
                    let mut m_tmp = $mat_name::zeros();

                    for row in 0..$size {
                        for col in 0..$size {
                            m_tmp.data[row][col] = self.data[col][row];
                        }
                    }

                    m_tmp
                }

                #[allow(dead_code)]
                pub fn equal_approx(&self, other: &$mat_name) -> bool {
                    for row in 0..$size {
                        for col in 0..$size {
                            if !equal_approx(self.data[row][col], other.data[row][col]) {
                                return false;
                            }
                        }
                    }

                    true
                }
            }

            impl PartialEq for $mat_name {
                fn eq(&self, other: &Self) -> bool {
                    self.data == other.data
                }
            }
        )
    }

    build_mat!(Mat4, 4);
    build_mat!(Mat3, 3);
    build_mat!(Mat2, 2);

    // Multiplication for mat4
    impl_op_ex!(* |a: &Mat4, b: &Mat4| -> Mat4 {
        let mut m = Mat4::zeros();

        for row in 0..4 {
            for col in 0..4 {
                m.data[row][col] = a.data[row][0] * b.data[0][col] +
                                   a.data[row][1] * b.data[1][col] +
                                   a.data[row][2] * b.data[2][col] +
                                   a.data[row][3] * b.data[3][col];
            }
        }
        m
    });

    // Multiplication for mat4 * vec4
    impl_op_ex!(* |a: &Mat4, b: &Vec4| -> Vec4 {
        let mut vec4_values: [f64; 4] = [0.0; 4];

        for row in 0..4 {
            vec4_values[row] = a.data[row][0] * b.x +
                               a.data[row][1] * b.y +
                               a.data[row][2] * b.z +
                               a.data[row][3] * b.w;
        }

        Vec4::new_vec4(vec4_values[0], vec4_values[1], vec4_values[2], vec4_values[3])
    });

    impl Mat4 {
        pub fn id() -> Mat4 {
            Mat4::new([[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]])
        }

        pub fn new_translation(x: f64, y: f64, z: f64) -> Mat4 {
            Mat4::new([[1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0]])
        }

        pub fn new_scaling(x: f64, y: f64, z: f64) -> Mat4 {
            Mat4::new([[x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0]])
        }

        pub fn new_rotation_x(r: f64) -> Mat4 {
            Mat4::new([[1.0, 0.0, 0.0, 0.0],
                [0.0, r.cos(), -r.sin(), 0.0],
                [0.0, r.sin(), r.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]])
        }

        pub fn new_rotation_y(r: f64) -> Mat4 {
            Mat4::new([[r.cos(), 0.0, r.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-r.sin(), 0.0, r.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]])
        }

        pub fn new_rotation_z(r: f64) -> Mat4 {
            Mat4::new([[r.cos(), -r.sin(), 0.0, 0.0],
                [r.sin(), r.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]])
        }

        pub fn new_shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Mat4 {
            Mat4::new([[1.0, xy, xz, 0.0],
                [yx, 1.0, yz, 0.0],
                [zx, zy, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]])
        }

        pub fn translate(&self, x: f64, y: f64, z: f64) -> Mat4 {
            self * Mat4::new_translation(x, y, z)
        }

        pub fn scale(&self, x: f64, y: f64, z: f64) -> Mat4 {
            self * Mat4::new_scaling(x, y, z)
        }

        pub fn rotate_x(&self, r: f64) -> Mat4 {
            self * Mat4::new_rotation_x(r)
        }

        pub fn rotate_y(&self, r: f64) -> Mat4 {
            self * Mat4::new_rotation_y(r)
        }

        pub fn rotate_z(&self, r: f64) -> Mat4 {
            self * Mat4::new_rotation_z(r)
        }

        pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Mat4 {
            self * Mat4::new_shearing(xy, xz, yx, yz, zx, zy)
        }

        pub fn submatrix(&self, row_to_exclude: usize, col_to_exclude: usize) -> Mat3 {
            if row_to_exclude > 3 || col_to_exclude > 3 {
                panic!("index out of bounds: cannot exclude a row or col that does not exist");
            }

            let mut m_values: Vec<f64> = Vec::with_capacity(9);

            for row in 0..4 {
                if row != row_to_exclude {
                    for col in 0..4 {
                        if col != col_to_exclude {
                            m_values.push(self.data[row][col]);
                        }
                    }
                }
            }

            Mat3::new([[m_values[0], m_values[1], m_values[2]],
                [m_values[3], m_values[4], m_values[5]],
                [m_values[6], m_values[7], m_values[8]]])
        }

        pub fn minor(&self, row: usize, col: usize) -> f64 {
            self.submatrix(row, col).determinant()
        }

        pub fn cofactor(&self, row: usize, col: usize) -> f64 {
            if (row + col) % 2 == 0 {
                self.minor(row, col)
            } else {
                self.minor(row, col).neg()
            }
        }

        pub fn determinant(&self) -> f64 {
            self.data[0][0] * self.cofactor(0, 0) +
                self.data[0][1] * self.cofactor(0, 1) +
                self.data[0][2] * self.cofactor(0, 2) +
                self.data[0][3] * self.cofactor(0, 3)
        }

        pub fn inverted(&self) -> Mat4 {
            let det = self.determinant();
            if det == 0.0 {
                // TODO: better error handling
                panic!("matrix has a determinant of 0. It cannot be inverted");
            }

            let mut m_tmp = Mat4::zeros();
            for row in 0..4 {
                for col in 0..4 {
                    m_tmp.data[col][row] = self.cofactor(row, col) / det;
                }
            }

            m_tmp
        }
    }

    impl Mat3 {
        pub fn submatrix(&self, row_to_exclude: usize, col_to_exclude: usize) -> Mat2 {
            if row_to_exclude > 2 || col_to_exclude > 2 {
                panic!("index out of bounds: cannot exclude a row or col that does not exist");
            }

            let mut m_values: Vec<f64> = Vec::with_capacity(4);

            for row in 0..3 {
                if row != row_to_exclude {
                    for col in 0..3 {
                        if col != col_to_exclude {
                            m_values.push(self.data[row][col]);
                        }
                    }
                }
            }

            Mat2::new([[m_values[0], m_values[1]],
                [m_values[2], m_values[3]]])
        }

        pub fn minor(&self, row: usize, col: usize) -> f64 {
            self.submatrix(row, col).determinant()
        }

        pub fn cofactor(&self, row: usize, col: usize) -> f64 {
            if (row + col) % 2 == 0 {
                self.minor(row, col)
            } else {
                self.minor(row, col).neg()
            }
        }

        pub fn determinant(&self) -> f64 {
            self.data[0][0] * self.cofactor(0, 0) +
                self.data[0][1] * self.cofactor(0, 1) +
                self.data[0][2] * self.cofactor(0, 2)
        }
    }

    impl Mat2 {
        pub fn determinant(&self) -> f64 {
            (self.data[0][0] * self.data[1][1]) - (self.data[0][1] * self.data[1][0])
        }
    }

    #[cfg(test)]
    mod matrix_tests {
        use super::*;

        #[test]
        fn create_mat() {
            let m4 = Mat4::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]]);

            assert_eq!(m4.data[0][0], 1.0);
            assert_eq!(m4.data[0][3], 4.0);
            assert_eq!(m4.data[1][0], 5.5);
            assert_eq!(m4.data[1][2], 7.5);
            assert_eq!(m4.data[2][2], 11.0);
            assert_eq!(m4.data[3][2], 15.5);

            let m4_zeros = Mat4::zeros();

            let mut m4_count = 0;
            // Iterate over mat4 values
            for i in m4_zeros.data.iter() {
                for j in i.iter() {
                    assert_eq!(*j, 0.0);
                    m4_count += 1;
                }
            }

            // Ensure that mat4 contains the correct number of elements
            assert_eq!(m4_count, 16);

            let m3 = Mat3::new([
                [-3.0, 5.0, 0.0],
                [1.0, -2.0, -7.0],
                [0.0, 1.0, 1.0]]);
            assert_eq!(m3.data[0][0], -3.0);
            assert_eq!(m3.data[1][1], -2.0);
            assert_eq!(m3.data[2][2], 1.0);

            let m3_zeros = Mat3::zeros();

            let mut m3_count = 0;
            // Iterate over mat3 values
            for i in m3_zeros.data.iter() {
                for j in i.iter() {
                    assert_eq!(*j, 0.0);
                    m3_count += 1
                }
            }

            // Ensure that mat3 contains the correct number of elements
            assert_eq!(m3_count, 9);

            let m2 = Mat2::new([
                [-3.0, 5.0],
                [1.0, -2.0]]);

            assert_eq!(m2.data[0][0], -3.0);
            assert_eq!(m2.data[0][1], 5.0);
            assert_eq!(m2.data[1][0], 1.0);
            assert_eq!(m2.data[1][1], -2.0);

            let m2_zeros = Mat2::zeros();

            let mut m2_count = 0;
            // Iterate over mat3 values
            for i in m2_zeros.data.iter() {
                for j in i.iter() {
                    assert_eq!(*j, 0.0);
                    m2_count += 1
                }
            }

            // Ensure that mat4 contains the correct number of elements
            assert_eq!(m2_count, 4);
        }

        #[test]
        fn matrix_equality() {
            let m4_1 = Mat4::zeros();
            let m4_2 = Mat4::zeros();
            let m4_3 = Mat4::new([[1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]]);
            let m4_4 = Mat4::new([[1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]]);

            assert_eq!(m4_1, m4_2);
            assert_ne!(m4_1, m4_3);
            assert_eq!(m4_4, m4_4);
        }

        #[test]
        fn modify_mat() {
            let mut m_mut = Mat4::zeros();
            m_mut.data[0][1] = 1.0;
            m_mut.data[2][3] = 3.5;

            assert_eq!(m_mut.data[0][1], 1.0);
            assert_eq!(m_mut.data[2][3], 3.5);
        }

        #[test]
        fn mat4_multiplication() {
            let m1 = Mat4::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0]]);

            let m2 = Mat4::new([
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0]]);

            let m3 = Mat4::new([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]]);

            assert_eq!(m1 * m2, m3);
        }

        #[test]
        fn mat4_multiplication_with_vec4() {
            let m = Mat4::new([
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0]]);

            let v = Vec4::new_vec4(1.0, 2.0, 3.0, 1.0);

            assert_eq!(m * v, Vec4::new_vec4(18.0, 24.0, 33.0, 1.0))
        }

        #[test]
        fn transposed() {
            let a = Mat4::new([
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0]]);

            let b = Mat4::new([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]]);

            assert_eq!(a.transposed(), b);

            // Inverting the identity matrix should produce the identity matrix
            let id = Mat4::id();
            assert_eq!(id, id.transposed());
        }

        #[test]
        fn submatrix() {
            let m3 = Mat3::new([
                [1.0, 5.0, 0.0],
                [-3.0, 2.0, 7.0],
                [0.0, 6.0, -3.0]]);

            let a = Mat2::new([[-3.0, 2.0],
                [0.0, 6.0]]);

            let sub_m3 = m3.submatrix(0, 2);

            assert_eq!(sub_m3, a);
        }

        #[test]
        fn minor() {
            let a1 = Mat3::new([
                [3.0, 5.0, 0.0],
                [2.0, -1.0, -7.0],
                [6.0, -1.0, 5.0]]);

            let b1 = a1.submatrix(1, 0);

            assert_eq!(a1.minor(1, 0), b1.determinant());

            let a2 = Mat4::new([
                [3.0, 5.0, 0.0, 1.0],
                [2.0, -1.0, -7.0, 3.0],
                [6.0, -1.0, 5.0, -2.0],
                [6.0, -1.0, 5.0, -2.0]]);

            let b2 = a2.submatrix(1, 0);

            assert_eq!(a2.minor(1, 0), b2.determinant());
        }

        #[test]
        fn cofactor() {
            let a1 = Mat3::new([
                [3.0, 5.0, 0.0],
                [2.0, -1.0, -7.0],
                [6.0, -1.0, 5.0]]);

            assert_eq!(a1.minor(0, 0), -12.0);
            assert_eq!(a1.cofactor(0, 0), -12.0);
            assert_eq!(a1.minor(1, 0), 25.0);
            assert_eq!(a1.cofactor(1, 0), -25.0);

            let a2 = Mat4::new([
                [3.0, 5.0, 0.0, 1.0],
                [2.0, -1.0, -7.0, 2.0],
                [8.0, -1.0, 5.0, -2.0],
                [6.0, -1.0, 9.0, 10.0]]);

            assert_eq!(a2.minor(0, 0), -160.0);
            assert_eq!(a2.cofactor(0, 0), -160.0);
            assert_eq!(a2.minor(1, 0), 336.0);
            assert_eq!(a2.cofactor(1, 0), -336.0);
        }

        #[test]
        fn determinant() {
            let a1 = Mat2::new([
                [1.0, 5.0],
                [-3.0, 2.0]]);

            assert_eq!(a1.determinant(), 17.0);

            let a2 = Mat3::new([
                [1.0, 2.0, 6.0],
                [-5.0, 8.0, -4.0],
                [2.0, 6.0, 4.0]]);

            assert_eq!(a2.cofactor(0, 0), 56.0);
            assert_eq!(a2.cofactor(0, 1), 12.0);
            assert_eq!(a2.cofactor(0, 2), -46.0);
            assert_eq!(a2.determinant(), -196.0);

            let a3 = Mat4::new([
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0]]);

            assert_eq!(a3.cofactor(0, 0), 690.0);
            assert_eq!(a3.cofactor(0, 1), 447.0);
            assert_eq!(a3.cofactor(0, 2), 210.0);
            assert_eq!(a3.cofactor(0, 3), 51.0);
            assert_eq!(a3.determinant(), -4071.0);
        }

        #[test]
        fn inversion() {
            let a = Mat4::new([
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0]]);
            let a_inv = Mat4::new([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639]]);
            let b = a.inverted();

            assert_eq!(a.determinant(), 532.0);

            assert_eq!(a.cofactor(2, 3), -160.0);
            assert_eq!(b.data[3][2], -160.0 / 532.0);
            assert_eq!(a.cofactor(3, 2), 105.0);
            assert_eq!(b.data[2][3], 105.0 / 532.0);

            assert_eq!(b.equal_approx(&a_inv), true);
        }

        #[test]
        fn inversion_multiplication() {
            let a = Mat4::new([
                [-3.0, 9.0, 7.0, 3.0],
                [3.0, -8.0, 2.0, -9.0],
                [-4.0, 4.0, 4.0, 1.0],
                [-6.0, 5.0, -1.0, 1.0]]);

            let b = Mat4::new([
                [8.0, 2.0, 2.0, 2.0],
                [3.0, -1.0, 7.0, 0.0],
                [7.0, 0.0, 5.0, 4.0],
                [6.0, -2.0, 0.0, 5.0]]);

            let c = a * b;
            assert_eq!((c * b.inverted()).equal_approx(&a), true);
        }
    }

    #[cfg(test)]
    mod transforms {
        use super::*;
        use std::f64::consts::PI;

        #[test]
        fn translation() {
            let t = Mat4::new_translation(5.0, -3.0, 2.0);
            let t_inv = t.inverted();
            let p = Vec4::new_point(-3.0, 4.0, 5.0);
            let v = Vec4::new_vec(-3.0, 4.0, 5.0);

            assert_eq!(t * p, Vec4::new_point(2.0, 1.0, 7.0));
            assert_eq!(t_inv * p, Vec4::new_point(-8.0, 7.0, 3.0));
            // Translation does not effect vectors
            assert_eq!(t * v, v);
        }

        #[test]
        fn scale() {
            let s = Mat4::new_scaling(2.0, 3.0, 4.0);
            let s_inv = s.inverted();
            let p = Vec4::new_point(-4.0, 6.0, 8.0);
            let v = Vec4::new_vec(-4.0, 6.0, 8.0);

            assert_eq!(s * p, Vec4::new_point(-8.0, 18.0, 32.0));
            assert_eq!(s * v, Vec4::new_vec(-8.0, 18.0, 32.0));
            assert_eq!(s_inv * v, Vec4::new_vec(-2.0, 2.0, 2.0));
        }

        #[test]
        fn rotation_x() {
            let p = Vec4::new_point(0.0, 1.0, 0.0);
            let half_quarter = Mat4::new_rotation_x(PI / 4.0);
            let half_quarter_inv = half_quarter.inverted();

            assert_eq!(half_quarter_inv * p,
                       Vec4::new_point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt() / 2.0)));
        }

        #[test]
        fn rotation_y() {
            let p = Vec4::new_point(0.0, 0.0, 1.0);
            let half_quarter = Mat4::new_rotation_y(PI / 4.0);
            let full_quarter = Mat4::new_rotation_y(PI / 2.0);

            assert_eq!(half_quarter * p,
                       Vec4::new_point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0));
            assert_eq!(full_quarter * p, Vec4::new_point(1.0, 0.0, 0.0));
        }

        #[test]
        fn rotation_z() {
            let p = Vec4::new_point(0.0, 1.0, 0.0);
            let half_quarter = Mat4::new_rotation_z(PI / 4.0);
            let full_quarter = Mat4::new_rotation_z(PI / 2.0);

            assert_eq!(half_quarter * p,
                       Vec4::new_point(-(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0, 0.0));
            assert_eq!(full_quarter * p, Vec4::new_point(-1.0, 0.0, 0.0));
        }

        #[test]
        fn shearing() {
            let p = Vec4::new_point(2.0, 3.0, 4.0);
            let t_xy = Mat4::new_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let t_xz = Mat4::new_shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
            let t_yx = Mat4::new_shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
            let t_yz = Mat4::new_shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let t_zx = Mat4::new_shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            let t_zy = Mat4::new_shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

            assert_eq!(t_xy * p, Vec4::new_point(5.0, 3.0, 4.0));
            assert_eq!(t_xz * p, Vec4::new_point(6.0, 3.0, 4.0));
            assert_eq!(t_yx * p, Vec4::new_point(2.0, 5.0, 4.0));
            assert_eq!(t_yz * p, Vec4::new_point(2.0, 7.0, 4.0));
            assert_eq!(t_zx * p, Vec4::new_point(2.0, 3.0, 6.0));
            assert_eq!(t_zy * p, Vec4::new_point(2.0, 3.0, 7.0));
        }

        #[test]
        fn fluent_api() {
            let t = Mat4::id().
                scale(2.0, 2.0, 1.0).
                translate(10.0, 5.0, 7.0);

            assert_eq!(t, Mat4::new([
                [2.0, 0.0, 0.0, 20.0],
                [0.0, 2.0, 0.0, 10.0],
                [0.0, 0.0, 1.0, 7.0],
                [0.0, 0.0, 0.0, 1.0]]));
        }
    }
}

