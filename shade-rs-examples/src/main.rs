mod glsl {
    pub fn concat<T>(v1: [T; 2], v2: [T; 2]) -> [T; 4]
    where
        T: Copy,
    {
        [v1[0], v1[1], v2[0], v2[1]]
    }

    pub type Float4 = [f32; 4];
    pub type Float = f32;

    pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
        [x, y, z, w]
    }

    pub fn sin(val: f32) -> f32 {
        val
    }
}

use glsl::{float4, sin};

#[shade_rs::fragment_shader]
fn fragment() -> &'static str {
    let mut x: glsl::Float4 = float4(0.0, 1.0, 2.0, 3.0);
    let y: glsl::Float = sin(10.0);
}

fn main() {
    let code = fragment();
    println!("{}", code);
}
