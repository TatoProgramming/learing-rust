extern crate image;
//  https://drive.google.com/drive/folders/14yayBb9XiL16lmuhbYhhvea8mKUUK77W
use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;

use std::path::Path;

struct Point {
    x: f32,
    y: f32,
    z: f32,
}
impl Copy for Point {}
impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}
impl Sub for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl Point {
    fn zero() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x: x, y: y, z: z }
    }
}

struct Color {
    r: f32,
    b: f32,
    g: f32,
    a: f32,
}
impl Color {
    fn new(r: f32, b: f32, g: f32, a: f32) -> Color {
        Color {
            r: r,
            b: b,
            g: g,
            a: a,
        }
    }
}
impl Copy for Color {}
impl Clone for Color {
    fn clone(&self) -> Color {
        *self
    }
}

#[derive(Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}
impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    fn make_unit_vector(self) -> Vector {
        let k = 1.0 / self.length();
        Vector::new( self.x * k, self.y * k, self.z * k)
    }
}

// dot product
impl Mul<Vector> for Vector {
    type Output = f32;
    fn mul(self, other: Vector) -> f32 {
        (self.x * other.x + self.y * other.y + self.z * other.z)
    }
}
impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, other: f32) -> Vector {
        Vector::new(self.x * other, self.y * other, self.z * other)
    }
}
impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

struct Sphere {
    radius: f32,
    center: Point,
}

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn display_color(color: &Color) {
    println!("({}, {}, {}, {})", color.r, color.g, color.b, color.a);
}

fn flatten(pixels: &mut Vec<Color>) -> Vec<u8> {
    const MAX_N: usize = WIDTH * HEIGHT * 4;
    let mut buffer = vec![0; MAX_N];
    for (i, color) in pixels.iter().enumerate() {
        let index = i * 4;
        buffer[index + 0] = (color.r * 255.99) as u8;
        buffer[index + 1] = (color.g * 255.99) as u8;
        buffer[index + 2] = (color.b * 255.99) as u8;
        buffer[index + 3] = (color.a * 255.99) as u8;
    }
    return buffer;
}

fn main() {
    let eye = Point::new(0.0, 0.0, 10.0);
    let mut pixels = vec![Color::new(0.0, 0.0, 0.0, 1.0); WIDTH * HEIGHT];

    for i in 0..pixels.len() {
        let x = i as i32 % 800;
        let y = i as i32 / 800;
        pixels[i].r = x as f32 / 800.0;
        pixels[i].g = y as f32 / 800.0;
        pixels[i].b = 0.2;
    }

    let a = Point::new(0.0, 0.0, 0.0);
    let b = Point::new(1.0, 1.0, 1.0);
    let p1 = b - a;
    let p2 = a - b;
    println!("{:?}", p1);
    println!("{:?}", p1.length());
    println!("{:?}", p1*3);

    // flatten the color vect to a buffer.
    let buffer = flatten(&mut pixels);
    // Save the buffer as "image.png"
    image::save_buffer(&Path::new("image.png"), &buffer, 800, 800, image::RGBA(8)).unwrap();
}
