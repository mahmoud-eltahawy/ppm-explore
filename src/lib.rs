use std::{
    array::from_fn,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const L: usize, V>(pub [V; L]);

impl<const L: usize, V: Default + Copy> Default for Vector<L, V> {
    fn default() -> Self {
        Self::splat(V::default())
    }
}

impl<const L: usize, V> Vector<L, V>
where
    V: Copy,
{
    pub const fn splat(v: V) -> Self {
        Self([v; L])
    }
    pub fn nth(&self, i: usize) -> Option<V> {
        if i < L { Some(self.0[i]) } else { None }
    }
    pub fn x(&self) -> V {
        self.0[0]
    }
    pub fn y(&self) -> V {
        self.0[1]
    }
}

impl<const L: usize, V: Mul<Output = V> + Copy + Into<f32>> Vector<L, V> {
    pub fn length(&self) -> f32 {
        self.0
            .iter()
            .map(|x| {
                let x: f32 = (*x * *x).into();
                x
            })
            .sum::<f32>()
            .sqrt()
    }
}

impl<V> Vector<3, V>
where
    V: Copy,
{
    pub fn z(&self) -> V {
        self.0[2]
    }
}

impl<V> Vector<4, V>
where
    V: Copy,
{
    pub fn z(&self) -> V {
        self.0[2]
    }
    pub fn w(&self) -> V {
        self.0[3]
    }
}

pub type Vec2<V> = Vector<2, V>;
pub type Vec3<V> = Vector<3, V>;
pub type Vec4<V> = Vector<4, V>;
pub type Vec2f32 = Vec2<f32>;
pub type Vec3f32 = Vec3<f32>;
pub type Vec4f32 = Vec4<f32>;
pub type Vec2f64 = Vec2<f64>;
pub type Vec3f64 = Vec3<f64>;
pub type Vec4f64 = Vec4<f64>;

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

impl<V> Vec4<V> {
    pub const fn new(x: V, y: V, z: V, w: V) -> Self {
        Self([x, y, z, w])
    }
}

union ArrayTransmute<T, const N: usize> {
    from: ManuallyDrop<[MaybeUninit<T>; N]>,
    to: ManuallyDrop<[T; N]>,
}

impl<const L: usize, V> Vector<L, V> {
    fn zip_map<F: Fn(V, V) -> V>(self, rhs: Self, f: F) -> Self {
        let mut res: [MaybeUninit<V>; L] = from_fn(|_| MaybeUninit::uninit());

        for (i, (a, b)) in self.0.into_iter().zip(rhs.0).enumerate() {
            res[i].write(f(a, b));
        }

        // SAFETY: Every element in `res` has been written to.
        let union = ArrayTransmute {
            from: ManuallyDrop::new(res),
        };
        Self(unsafe { ManuallyDrop::into_inner(union.to) })
    }
}

impl<const L: usize, V: Add<Output = V>> Add for Vector<L, V> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, V::add)
    }
}

impl<const L: usize, V: Sub<Output = V>> Sub for Vector<L, V> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, V::sub)
    }
}

impl<const L: usize, V: Div<Output = V>> Div for Vector<L, V> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, V::div)
    }
}

impl<const L: usize, V: Mul<Output = V>> Mul for Vector<L, V> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, V::mul)
    }
}
