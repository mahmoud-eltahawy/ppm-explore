use std::{
    array::from_fn,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const L: usize, V>([V; L]);

impl<const L: usize, V: Default + Copy> Default for Vector<L, V> {
    fn default() -> Self {
        Self::splat(V::default())
    }
}

impl<const L: usize, V> Vector<L, V>
where
    V: Copy,
{
    pub const fn get(&self, i: usize) -> Option<V> {
        if i < L { Some(self.0[i]) } else { None }
    }
    pub const fn splat(v: V) -> Self {
        Self([v; L])
    }
    pub const fn inner(self) -> [V; L] {
        let Self(arr) = self;
        arr
    }
}

impl<const L: usize, V: Mul<Output = V> + Copy + Into<f64>> Vector<L, V> {
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }
    pub fn length_sq(&self) -> f64 {
        self.0
            .iter()
            .map(|x| {
                let x: f64 = (*x * *x).into();
                x
            })
            .sum::<f64>()
    }
}

impl<V> Vector<1, V>
where
    V: Copy,
{
    pub const fn x(&self) -> V {
        self.0[0]
    }
}

impl<V> Vector<2, V>
where
    V: Copy,
{
    pub const fn x(&self) -> V {
        self.0[0]
    }
    pub const fn y(&self) -> V {
        self.0[1]
    }
}

impl<V> Vector<3, V>
where
    V: Copy,
{
    pub const fn x(&self) -> V {
        self.0[0]
    }
    pub const fn y(&self) -> V {
        self.0[1]
    }
    pub const fn z(&self) -> V {
        self.0[2]
    }
}

impl<V> Vector<4, V>
where
    V: Copy,
{
    pub const fn x(&self) -> V {
        self.0[0]
    }
    pub const fn y(&self) -> V {
        self.0[1]
    }
    pub const fn z(&self) -> V {
        self.0[2]
    }
    pub const fn w(&self) -> V {
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
    unsafe fn from_exact_iter(iter: impl IntoIterator<Item = V>) -> Self {
        let mut res: [MaybeUninit<V>; L] = from_fn(|_| MaybeUninit::uninit());
        for (i, val) in iter.into_iter().enumerate() {
            debug_assert!(i < L, "iterator produced more than L items");
            res[i].write(val);
        }
        // SAFETY: we trust the caller that exactly L items were written.
        let union = ArrayTransmute {
            from: ManuallyDrop::new(res),
        };
        unsafe { Self(ManuallyDrop::into_inner(union.to)) }
    }
}

impl<const L: usize, V> Vector<L, V> {
    fn zip_map<F: Fn(V, V) -> V>(self, rhs: Self, f: F) -> Self {
        unsafe { Self::from_exact_iter(self.0.into_iter().zip(rhs.0).map(|(a, b)| f(a, b))) }
    }
}

impl<const L: usize, V> Vector<L, V> {
    fn map<F: Fn(V) -> V>(self, f: F) -> Self {
        unsafe { Self::from_exact_iter(self.0.into_iter().map(f)) }
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

impl<const L: usize, V: Neg<Output = V>> Neg for Vector<L, V> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(|a| -a)
    }
}

impl<const L: usize, V: Mul<Output = V> + Copy> Mul<V> for Vector<L, V> {
    type Output = Self;
    fn mul(self, scalar: V) -> Self::Output {
        self.map(|a| a * scalar)
    }
}

impl<const L: usize, V: Div<Output = V> + Copy> Div<V> for Vector<L, V> {
    type Output = Self;
    fn div(self, scalar: V) -> Self::Output {
        self.map(|a| a / scalar)
    }
}

impl<const L: usize, V: Mul<Output = V> + AddAssign + Copy> Vector<L, V> {
    pub fn dot(self, rhs: Self) -> V {
        let product = self * rhs;
        let mut iter = product.0.into_iter();
        let first = iter.next().unwrap(); // L is always >= 1?
        let mut acc = first;
        for x in iter {
            acc += x;
        }
        acc
    }
}
