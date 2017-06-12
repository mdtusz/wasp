
use core::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x: x, y: y, z: z }
    }

    /*
    pub fn distance_from_origin(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn distance(&self, other: Point3) -> f32 {
        (self - other).distance_from_origin()
    }
    */
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Point3 {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Point3 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        Point3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div for Point3 {
    type Output = Point3;

    fn div(self, other: Point3) -> Point3 {
        Point3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
