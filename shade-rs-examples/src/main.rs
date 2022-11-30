#[shade_rs::module]
mod shaders {
    #[fragment_shader]
    pub fn fragment(
        #[input] input_color: Float4,
        #[output] output_color: &mut Float4,
        #[uniform(location = 0)] color_shift: Float,
        #[uniform(location = 1)] n_iterations: Int,
    ) {
        let mut color: Float4 = input_color;

        for i in 0..n_iterations {
            if i % 2 == 0 {
                *color.r_mut() += color_shift;
            } else {
                *color.b_mut() += color_shift;
            }
        }

        *output_color = color;
    }
}

use shade_rs::*;

fn main() {
    let code = shaders::fragment::code();
    println!("{}", code);

    let input = float4(0.0, 0.0, 0.0, 255.0);
    let mut output = float4(0.0, 0.0, 0.0, 0.0);
    shaders::fragment::call(input, &mut output, 0.01, 2);
    assert_eq!(output, float4(0.01, 0.0, 0.01, 255.0));
}
