use crate::code_utils::{rebuild_code, Indentable};
use crate::convenience_wrap::ConvenienceWrap;
use crate::mapper::Mapper;
use quote::ToTokens;
use syn::visit::Visit;
use syn::{Expr, ItemFn, Pat, Type};

pub struct ShaderFnParser {
    pub code: String,
}

impl ConvenienceWrap for ShaderFnParser {}

impl<'ast> Visit<'ast> for ShaderFnParser {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let shader_code = Self::new().apply(|p| p.visit_block(&i.block)).code;

        self.code.push_str("int main() {\n");
        self.code.push_str(&shader_code.reindent(4));
        self.code.push_str("\n}");
    }

    fn visit_local(&mut self, i: &'ast syn::Local) {
        if let Pat::Type(type_pattern) = &i.pat {
            self.visit_type(&type_pattern.ty);
            self.code.push_str(" ");

            self.visit_pat(&type_pattern.pat);
        } else {
            panic!(
                "Unsupported variable initializtion syntax: {:?}. Should be 'let var_name: VarType = ...'",
                rebuild_code(&i)
            )
        }

        if let Some((_, expr)) = &i.init {
            self.code.push_str(" = ");
            self.visit_expr(&expr);
        }

        self.code.push_str(";\n");
    }

    fn visit_type(&mut self, i: &'ast Type) {
        if let Type::Path(path) = &i {
            let rust_type = path.path.segments.last().unwrap().ident.to_string();
            let glsl_type = Mapper::translate_type(&rust_type);
            self.code.push_str(glsl_type);
        } else {
            panic!("unknown variable type");
        }
    }

    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        self.visit_expr(&i.func);

        self.code.push_str("(");

        let mut args_iter = i.args.iter();
        if let Some(expr) = args_iter.next() {
            self.visit_expr(&expr);
        }

        for expr in args_iter {
            self.code.push_str(", ");
            self.visit_expr(&expr);
        }

        self.code.push_str(")");
    }

    fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
        let Expr::Range(rng) = &*i.expr else {
            panic!("unsupported loop range syntax: {} ({})", rebuild_code(&i), rebuild_code(&i.expr));
        };

        let left = rebuild_code(&rng.from.as_ref().unwrap());
        let right = rebuild_code(&rng.to.as_ref().unwrap());

        let Pat::Ident(identifier) = &i.pat else {
            panic!("unsupported variable binding in for loop syntax: {}", rebuild_code(&i.pat));
        };
        let identifier_name = Self::new().apply(|p| p.visit_ident(&identifier.ident)).code;

        let loop_body = Self::new().apply(|p| p.visit_block(&i.body)).code;

        let code = format!(
            r#"
                for (int {identifier_name} = {left}; {identifier_name} < {right}; ++{identifier_name}) {{
                    {loop_body}
                }}
            "#
        ).remove_indent();
        self.code.push_str(&code);
        self.code.push_str("\n");
    }

    fn visit_stmt(&mut self, i: &'ast syn::Stmt) {
        syn::visit::visit_stmt(self, i);
        //self.code.push_str(";\n");
    }

    fn visit_expr_binary(&mut self, i: &'ast syn::ExprBinary) {
        self.visit_expr(&i.left);

        self.visit_bin_op(&i.op);

        self.visit_expr(&i.right);
    }

    fn visit_bin_op(&mut self, i: &'ast syn::BinOp) {
        let op_str: String = i
            .to_token_stream()
            .into_iter()
            .map(|token| token.to_string())
            .collect();

        self.code.push_str(&format!(" {} ", op_str));
    }

    fn visit_expr_assign(&mut self, i: &'ast syn::ExprAssign) {
        self.visit_expr(&i.left);

        self.code.push_str(" = ");

        self.visit_expr(&i.right);

        self.code.push_str(";\n");
    }

    fn visit_path(&mut self, i: &'ast syn::Path) {
        let rust_name = i.segments.last().unwrap().ident.to_string();
        let glsl_name = Mapper::translate_fun(&rust_name).unwrap_or(&rust_name);

        self.code.push_str(glsl_name);
    }

    fn visit_ident(&mut self, i: &'ast syn::Ident) {
        let rust_name = i.to_string();
        let glsl_name = Mapper::translate_fun(&rust_name).unwrap_or(&rust_name);

        self.code.push_str(glsl_name);
    }

    fn visit_lit_float(&mut self, i: &'ast syn::LitFloat) {
        self.code.push_str(&i.to_string());
    }
}

impl ShaderFnParser {
    pub fn new() -> ShaderFnParser {
        ShaderFnParser {
            code: String::new(),
        }
    }
}
