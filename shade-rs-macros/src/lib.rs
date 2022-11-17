use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse_macro_input;
use syn::visit::Visit;
use syn::ItemFn;

struct FnVisitor {
    code: String,
}

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.visit_block(&node.block);
    }

    fn visit_local(&mut self, i: &'ast syn::Local) {
        self.code.push_str("TYPE ");
        self.visit_pat(&i.pat);

        if let Some((_, expr)) = &i.init {
            self.code.push_str(" = ");
            self.visit_expr(&expr);
        }

        self.code.push_str(";\n");
    }

    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        self.visit_expr(&i.func);

        self.code.push_str("(");

        let mut iter = i.args.iter();
        if let Some(expr) = iter.next() {
            self.visit_expr(&expr);
        }

        for expr in iter {
            self.code.push_str(", ");
            self.visit_expr(&expr);
        }

        self.code.push_str(")");
    }

    fn visit_lit_float(&mut self, i: &'ast syn::LitFloat) {
        self.code.push_str(&i.to_string());
    }

    fn visit_ident(&mut self, i: &'ast syn::Ident) {
        self.code.push_str(&i.to_string());
        syn::visit::visit_ident(self, i);
    }

    fn visit_expr_binary(&mut self, i: &'ast syn::ExprBinary) {
        self.visit_expr(&i.left);

        self.visit_bin_op(&i.op);

        self.visit_expr(&i.right);
    }

    fn visit_stmt(&mut self, i: &'ast syn::Stmt) {
        syn::visit::visit_stmt(self, i);
        self.code.push_str(";\n");
    }

    fn visit_expr_assign(&mut self, i: &'ast syn::ExprAssign) {
        self.visit_expr(&i.left);

        self.code.push_str(" = ");

        self.visit_expr(&i.right);

        self.code.push_str(";\n");
    }

    fn visit_bin_op(&mut self, i: &'ast syn::BinOp) {
        let op_str = i.to_token_stream().into_iter().next().unwrap().to_string();

        self.code.push_str(&format!(" {} ", op_str));
    }
}

impl FnVisitor {
    fn new() -> FnVisitor {
        FnVisitor {
            code: String::new(),
        }
    }
}

#[proc_macro_attribute]
pub fn fragment_shader(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);

    let fun_name = format_ident!("{}", function.sig.ident.to_string());

    let mut visitor = FnVisitor::new();
    visitor.visit_item_fn(&function);
    let code: &str = &visitor.code;

    let block = &function.block;

    let expanded = quote! {
        fn #fun_name() -> &'static str {
            fn _orig_func_just_for_compiler_checks()
            #block

            #code
        }
    };

    TokenStream::from(expanded)
}