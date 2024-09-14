use core::fmt;
use std::{
    fmt::{Display, Formatter},
    ops::{Add, Deref, Div, Sub},
};


use std::simd::f32x4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn unit(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    const fn _add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    const fn _sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    const fn _sub_de(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn abs_difference(&self, other: &Self) -> Self {
        Self {
            x: (self.x - other.x).abs(),
            y: (self.y - other.y).abs(),
            z: (self.z - other.z).abs(),
        }
    }

    pub fn vec_division(&self, other: &Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    pub const fn scale(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn angle_between(&self, a: &Self, c: &Self) -> f32 {
        let m = (a - self).unit();
        let n = (c - self).unit();
        let r = m.dot(n);

        r.acos().to_degrees()
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub const fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    pub const fn basis() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    pub fn normalize_with(&self, other: Self) -> Self {
        *self / other
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &'b Vector3) -> Vector3 {
        self._sub(rhs)
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        self._sub_de(rhs)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self._add(&rhs)
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.vec_division(&rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum Color {
    Init = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
}

impl Color {
    pub fn from_id(id: i32) -> Self {
        match id {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            _ => panic!("Invalid color id: {}", id),
        }
    }
    pub fn next(self) -> Self {
        match self {
            Self::Init => Self::A,
            Self::A => Self::B,
            Self::B => Self::C,
            Self::C => Self::D,
            Self::D => Self::A,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sat(pub u64);

impl Sat {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Default for Sat {
    fn default() -> Self {
        Self(0)
    }
}

impl Display for Sat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct User(pub u64);

impl User {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for User {
    fn default() -> Self {
        Self(0)
    }
}
