use std::ops::{Add, AddAssign, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(arr: [T; N]) -> Self {
        Self { data: arr }
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: AddAssign,
    T: Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[0] += rhs.data[0];
        self.data[0] += rhs.data[0];
        self.data[0] += rhs.data[0];
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: AddAssign,
    T: Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

pub type Int = i32;
pub type Float = f32;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::From,
    derive_more::Index,
    derive_more::IndexMut,
)]
#[from(forward)]
pub struct Float4(Vector<f32, 4>);

impl Float4 {
    pub fn r(&self) -> f32 {
        self[0]
    }

    pub fn g(&self) -> f32 {
        self[1]
    }

    pub fn b(&self) -> f32 {
        self[2]
    }

    pub fn a(&self) -> f32 {
        self[3]
    }

    pub fn r_mut(&mut self) -> &mut f32 {
        &mut self[0]
    }

    pub fn g_mut(&mut self) -> &mut f32 {
        &mut self[1]
    }

    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self[2]
    }

    pub fn a_mut(&mut self) -> &mut f32 {
        &mut self[3]
    }
}

fn itof(i: Int) -> Float {
    i as Float
}

fn ftoi(f: Float) -> Int {
    f as Int
}

pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
    [x, y, z, w].into()
}

pub fn sin(val: Float) -> Float {
    val.sin()
}

pub fn cos(val: Float) -> Float {
    val.cos()
}
