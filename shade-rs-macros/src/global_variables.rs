use shade_rs_core::parsing_error;
use syn::spanned::Spanned;

#[derive(Debug)]
pub enum GlobalVariableKind {
    // TODO: add Attribute
    Uniform,
    Input,
    Output,
}

#[derive(Debug)]
pub struct GlobalVariableDefinition {
    pub kind: GlobalVariableKind,
    pub location: Option<i32>,
}

impl GlobalVariableKind {
    fn from_ident(ident: &syn::Ident) -> GlobalVariableKind {
        match ident.to_string().as_str() {
            "uniform" => GlobalVariableKind::Uniform,
            "input" => GlobalVariableKind::Input,
            "output" => GlobalVariableKind::Output,
            other => parsing_error!(ident, "unsupported argument attribute type: '{}'", other),
        }
    }
}

pub fn parse_global_var(attr: &syn::Attribute) -> GlobalVariableDefinition {
    let meta = match attr.parse_meta() {
        Ok(meta) => meta,
        Err(err) => parsing_error!(err.span(), "{}", err.to_string()),
    };

    match meta {
        syn::Meta::Path(path) => {
            let kind = GlobalVariableKind::from_ident(&path.segments.last().unwrap().ident);
            GlobalVariableDefinition {
                kind,
                location: None,
            }
        }
        syn::Meta::List(list) => {
            let kind = GlobalVariableKind::from_ident(&list.path.segments.last().unwrap().ident);

            let location = list.nested.first().map(|location_def| {
                let syn::NestedMeta::Meta(syn::Meta::NameValue(key_value)) = location_def else {
                    parsing_error!(location_def, "expected key=value syntax");
                };

                if key_value.path.segments.last().unwrap().ident.to_string() != "location" {
                    parsing_error!(
                        key_value.path,
                        "unsupported parameter name. Expected 'location'"
                    );
                }

                let syn::Lit::Int(location) = &key_value.lit else {
                    parsing_error!(key_value.lit, "unsupported parameter value type for 'location'. Expected integer literal");
                };

                location
                    .base10_parse::<i32>()
                    .unwrap_or_else(|err| parsing_error!(location, "{}", err.to_string()))
            });

            GlobalVariableDefinition { kind, location }
        }
        syn::Meta::NameValue(_) => parsing_error!(meta, "unsupported attribute syntax"),
    }
}
