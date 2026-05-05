use std::ops::{Add, Div, Mul, Sub};

pub struct Vector<const L: usize>(pub [f32; L]);

impl<const L: usize> Vector<L> {
    pub const fn init_with(v: f32) -> Self {
        Self([v; L])
    }
    pub fn nth(&self, i: usize) -> f32 {
        self.0[i]
    }
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    pub fn y(&self) -> f32 {
        self.0[1]
    }
    pub fn z(&self) -> f32 {
        assert_eq!(L, 3, "cannot access z in vec2 or less");
        self.0[2]
    }
}

pub type Vec2 = Vector<2>;
pub type Vec3 = Vector<3>;

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self([x, y])
    }
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}

impl<const L: usize> Add for Vector<L> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len(), "vectors must be the same size");
        let mut res: [f32; L] = [0.0; L];
        for (i, (a, b)) in self.0.iter().zip(rhs.0).enumerate() {
            res[i] = a + b;
        }
        Self(res)
    }
}

impl<const L: usize> Sub for Vector<L> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len(), "vectors must be the same size");
        let mut res: [f32; L] = [0.0; L];
        for (i, (a, b)) in self.0.iter().zip(rhs.0).enumerate() {
            res[i] = a - b;
        }
        Self(res)
    }
}

impl<const L: usize> Div for Vector<L> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len(), "vectors must be the same size");
        let mut res: [f32; L] = [0.0; L];
        for (i, (a, b)) in self.0.iter().zip(rhs.0).enumerate() {
            res[i] = a / b;
        }
        Self(res)
    }
}

impl<const L: usize> Mul for Vector<L> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len(), "vectors must be the same size");
        let mut res: [f32; L] = [0.0; L];
        for (i, (a, b)) in self.0.iter().zip(rhs.0).enumerate() {
            res[i] = a * b;
        }
        Self(res)
    }
}

impl<const L: usize> Vector<L> {
    pub fn length(&self) -> f32 {
        self.0.map(|x| x * x).iter().sum::<f32>().sqrt()
    }
}
