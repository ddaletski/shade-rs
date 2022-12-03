mod dtypes;
mod parsing_utils;

pub use dtypes::numeric::*;
pub use dtypes::vector::*;

pub use dtypes::bool_n::*;
pub use dtypes::float_n::*;
pub use dtypes::int_n::*;
pub use dtypes::uint_n::*;

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
