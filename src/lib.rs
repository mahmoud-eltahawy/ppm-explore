use std::ops::{Add, Div, Mul, Sub};

pub struct Vector<const L: usize, V>(pub [V; L]);

impl<const L: usize, V> Vector<L, V>
where
    V: Mul<Output = V> + Clone + Copy + Default + Into<f32>,
{
    pub const fn splat(v: V) -> Self {
        Self([v; L])
    }
    pub fn nth(&self, i: usize) -> V {
        self.0[i]
    }
    pub fn x(&self) -> V {
        self.0[0]
    }
    pub fn y(&self) -> V {
        self.0[1]
    }
    pub fn z(&self) -> V {
        assert_eq!(L, 3, "cannot access z in vec2 or less");
        self.0[2]
    }
    pub fn w(&self) -> V {
        assert_eq!(L, 4, "cannot access z in vec3 or less");
        self.0[3]
    }
    pub fn length(&self) -> f32 {
        self.0
            .map(|x| {
                let x: f32 = (x * x).into();
                x
            })
            .iter()
            .sum::<f32>()
            .sqrt()
    }
}

pub type Vec2<V> = Vector<2, V>;
pub type Vec3<V> = Vector<3, V>;
pub type Vec2f32 = Vec2<f32>;
pub type Vec3f32 = Vec3<f32>;
pub type Vec2f64 = Vec2<f64>;
pub type Vec3f64 = Vec3<f64>;

impl<V> Vec2<V> {
    pub const fn new(x: V, y: V) -> Self {
        Self([x, y])
    }
}

impl<V> Vec3<V> {
    pub const fn new(x: V, y: V, z: V) -> Self {
        Self([x, y, z])
    }
}

impl<const L: usize, V: Add<Output = V> + Clone + Copy + Default> Add for Vector<L, V> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len(), "vectors must be the same size");
        let mut res: [V; L] = [V::default(); L];
        for (i, (a, b)) in self.0.iter().zip(rhs.0).enumerate() {
            res[i] = *a + b;
        }
        Self(res)
    }
}

impl<const L: usize, V: Sub<Output = V> + Clone + Copy + Default> Sub for Vector<L, V> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len(), "vectors must be the same size");
        let mut res = [V::default(); L];
        for (i, (a, b)) in self.0.iter().zip(rhs.0).enumerate() {
            res[i] = *a - b;
        }
        Self(res)
    }
}

impl<const L: usize, V: Div<Output = V> + Clone + Copy + Default> Div for Vector<L, V> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len(), "vectors must be the same size");
        let mut res = [V::default(); L];
        for (i, (a, b)) in self.0.iter().zip(rhs.0).enumerate() {
            res[i] = *a / b;
        }
        Self(res)
    }
}

impl<const L: usize, V: Mul<Output = V> + Clone + Copy + Default> Mul for Vector<L, V> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len(), "vectors must be the same size");
        let mut res = [V::default(); L];
        for (i, (a, b)) in self.0.iter().zip(rhs.0).enumerate() {
            res[i] = *a * b;
        }
        Self(res)
    }
}
