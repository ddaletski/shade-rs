#[shade_rs::module]
mod shaders {
    fn call() {}

    #[fragment_shader]
    pub fn Fragment(
        #[uniform(location = 0)] int_uniform: Int,
        #[uniform(location = 1)] float_uniform: Float,
    ) {
        let mut x: Float4 = float4(1.0, 2.0, 3.0, 4.0);
        let y: Float4 = float4(0.0, 1.0, 2.0, 3.0);

        let mut a: Float = sin(10.0);

        x = x + y;
        x += y;

        for _i in 1..10 {
            a += float_uniform + int_to_float(int_uniform);
        }

        if 1 > 0 {
            2;
        } else if false {
            6;
        } else {
            7;
        }

        call();
    }
}

fn main() {
    let code = shaders::Fragment::code();
    println!("{}", code);

    shaders::Fragment::call(0, 1.0);
}
