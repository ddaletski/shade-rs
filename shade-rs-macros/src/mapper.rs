pub struct Mapper {}

impl Mapper {
    pub fn translate_type(rust_type: &str) -> String {
        match rust_type {
            "Int" => "int",
            "Float" => "float",
            "Float2" => "vec2",
            "Float3" => "vec3",
            "Float4" => "vec4",
            t => panic!("unknown type: {}", t),
        }
        .to_owned()
    }

    pub fn translate_fun(rust_fun: &str) -> String {
        match rust_fun {
            "float2" => "vec2",
            "float3" => "vec3",
            "float4" => "vec4",
            "r_mut" => "r",
            "g_mut" => "g",
            "b_mut" => "b",
            "a_mut" => "a",
            x => x,
        }
        .to_owned()
    }
}
