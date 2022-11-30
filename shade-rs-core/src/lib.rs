mod dtypes;
mod parsing_utils;

pub use dtypes::*;
pub use parsing_utils::*;

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
