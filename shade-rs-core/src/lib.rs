use std::ops::{Add, AddAssign};

type Scalar<T, const N: usize> = [T; N];

#[derive(Debug, Clone, Copy)]
pub struct Float4(Scalar<f32, 4>);
impl From<Scalar<f32, 4>> for Float4 {
    fn from(val: Scalar<f32, 4>) -> Self {
        Self(val)
    }
}

impl Float4 {
    pub fn red(&self) -> f32 {
        self.0[0]
    }
    pub fn green(&self) -> f32 {
        self.0[1]
    }
    pub fn blue(&self) -> f32 {
        self.0[2]
    }
    pub fn alpha(&self) -> f32 {
        self.0[3]
    }

    pub fn red_mut(&mut self) -> &mut f32 {
        &mut self.0[0]
    }
    pub fn green_mut(&mut self) -> &mut f32 {
        &mut self.0[1]
    }
    pub fn blue_mut(&mut self) -> &mut f32 {
        &mut self.0[2]
    }
    pub fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.0[3]
    }
}

pub type Float = f32;
pub type Int = i32;

fn int_to_float(i: Int) -> Float {
    i as Float
}

fn float_to_int(f: Float) -> Int {
    f as Int
}

pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
    [x, y, z, w].into()
}

pub fn float(x: f32) -> Float {
    x
}

pub fn sin(val: Float) -> Float {
    val
}

impl AddAssign for Float4 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
        self.0[3] += rhs.0[3];
    }
}

impl Add for Float4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}
