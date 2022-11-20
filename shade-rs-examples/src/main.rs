mod glsl {
    use std::ops::{Add, AddAssign};

    type Scalar<T, const N: usize> = [T; N];

    #[derive(Debug, Clone, Copy)]
    pub struct Float4(Scalar<f32, 4>);
    impl From<Scalar<f32, 4>> for Float4 {
        fn from(val: Scalar<f32, 4>) -> Self {
            Self(val)
        }
    }

    pub type Float = f32;

    pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
        [x, y, z, w].into()
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
}

use glsl::{float4, sin, Float, Float4};

fn call() {
}

#[shade_rs::fragment_shader]
fn fragment() -> &'static str {
    let a: Float4 = float4(1.0, 2.0, 3.0, 4.0);
    let mut x: Float4 = float4(0.0, 1.0, 2.0, 3.0);
    let mut y: Float = sin(10.0);

    x = x + a;

    for i in 1..10 {
        y += 1.0;
    }

    if 1 > 0 {
        2;
    }

    if 1 > 0 {
        2;
    } else {
        3;
    }

    if 1 > 0 {
        2;
    } else if 3 > 0 {
        4;
    } else if 5 > 0 {
        6;
    } else {
        7;
    }

    call();

    x += x;
}

fn main() {
    let code = fragment();
    println!("{}", code);
}
