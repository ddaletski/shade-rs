use crate::code_utils::{rebuild_code, Indentable};
use crate::convenience_wrap::ConvenienceWrap;
use crate::mapper::Mapper;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Expr, ItemFn, Pat, Stmt, Type};

const INDENT_WIDTH: usize = 4;

macro_rules! parsing_error {
    ($tree:expr, $($fmt_args:expr),+) => {
        let msg = format!($($fmt_args),*);
        let span = $tree.span().unwrap();
        span.error(msg).emit();
        unreachable!(); // for unstructuring let to see that this function always panics
    };
}

pub struct ShaderFnParser {
    pub code: String,
}

impl ConvenienceWrap for ShaderFnParser {}

impl<'ast> Visit<'ast> for ShaderFnParser {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let shader_code = Self::new().apply(|p| p.visit_block(&i.block)).code;

        self.code.push_str("int main()\n");
        self.code.push_str(&shader_code);
    }

    fn visit_local(&mut self, i: &'ast syn::Local) {
        if let Pat::Type(type_pattern) = &i.pat {
            self.visit_type(&type_pattern.ty);
            self.code.push_str(" ");

            self.visit_pat(&type_pattern.pat);
        } else {
            parsing_error!(
                i.pat,
                "Unsupported variable initializtion syntax. Should be 'let [mut] var_name: VarType = ...'"
            );
        }

        if let Some((_, expr)) = &i.init {
            self.code.push_str(" = ");
            self.visit_expr(&expr);
        }
    }

    fn visit_type(&mut self, i: &'ast Type) {
        if let Type::Path(path) = &i {
            let rust_type = path.path.segments.last().unwrap().ident.to_string();
            let glsl_type = Mapper::translate_type(&rust_type);
            self.code.push_str(glsl_type);
        } else {
            parsing_error!(i, "unknown variable type");
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
            parsing_error!(i.expr, "unsupported loop iterator. Expected range object (a..b | a..=b)");
        };

        let left = rebuild_code(&rng.from.as_ref().unwrap());
        let right = rebuild_code(&rng.to.as_ref().unwrap());

        let Pat::Ident(identifier) = &i.pat else {
            parsing_error!(i.pat, "unsupported variable binding in for loop syntax. Expected `for <var_name> in <range>");
        };
        let identifier_name = Self::new().apply(|p| p.visit_ident(&identifier.ident)).code;

        let loop_body = Self::new().apply(|p| p.visit_block(&i.body)).code;

        let code = format!("for (int {identifier_name} = {left}; {identifier_name} < {right}; ++{identifier_name}) {loop_body}");
        self.code.push_str(&code);
        self.code.push_str("\n");
    }

    fn visit_block(&mut self, i: &'ast syn::Block) {
        self.code.push_str("{\n");

        let block_code = Self::new()
            .apply(|parser| {
                for stmt in &i.stmts {
                    parser.visit_stmt(&stmt);
                }
            })
            .code
            .reindent(INDENT_WIDTH);

        self.code.push_str(&block_code);

        self.code.push_str("\n}");
    }

    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        self.code.push_str("if (");
        self.visit_expr(&i.cond);
        self.code.push_str(") ");
        self.visit_block(&i.then_branch);

        if let Some((_, else_branch)) = &i.else_branch {
            self.code.push_str(" else ");
            self.visit_expr(else_branch);
            //self.code.push_str("");
        }

        self.code.push_str("\n");
    }

    fn visit_stmt(&mut self, i: &'ast syn::Stmt) {
        syn::visit::visit_stmt(self, i);
        match &i {
            Stmt::Local(_) | Stmt::Semi(_, _) => {
                self.code.push_str(";\n");
            }
            _ => {}
        }
    }

    fn visit_expr_binary(&mut self, i: &'ast syn::ExprBinary) {
        self.visit_expr(&i.left);

        self.code.push_str(" ");
        self.visit_bin_op(&i.op);
        self.code.push_str(" ");

        self.visit_expr(&i.right);
    }

    fn visit_bin_op(&mut self, i: &'ast syn::BinOp) {
        self.code.push_str(&rebuild_code(&i));
    }

    fn visit_expr_assign(&mut self, i: &'ast syn::ExprAssign) {
        self.visit_expr(&i.left);

        self.code.push_str(" = ");

        self.visit_expr(&i.right);
    }

    fn visit_expr_assign_op(&mut self, i: &'ast syn::ExprAssignOp) {
        self.visit_expr(&i.left);

        self.code.push_str(" ");
        self.code.push_str(&rebuild_code(&i.op));
        self.code.push_str(" ");

        self.visit_expr(&i.right);
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

    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        self.code.push_str(&rebuild_code(&i.lit));
    }
}

impl ShaderFnParser {
    pub fn new() -> ShaderFnParser {
        ShaderFnParser {
            code: String::new(),
        }
    }
}
