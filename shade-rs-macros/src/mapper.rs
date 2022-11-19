pub struct Mapper {}

impl Mapper {
    pub fn translate_type(rust_type: &str) -> &'static str {
        match rust_type {
            "Float" => "float",
            "Float2" => "vec2",
            "Float3" => "vec3",
            "Float4" => "vec4",
            _ => unreachable!(),
        }
    }

    pub fn translate_fun(rust_fun: &str) -> Option<&'static str> {
        match rust_fun {
            "float2" => Some("vec2"),
            "float3" => Some("vec3"),
            "float4" => Some("vec4"),
            _ => None,
        }
    }
}
