use quote::{quote, ToTokens};

pub fn rebuild_code<T: ToTokens>(token_tree: &T) -> String {
    quote! {
        #token_tree
    }
    .to_string()
}

pub trait Indentable {
    fn remove_indent(self) -> String;
    fn reindent(self, indentation: usize) -> String;
}

impl Indentable for &str {
    fn remove_indent(self) -> String {
        let lines: Vec<_> = self.lines().filter(|line| line.trim() != "").collect();

        let Some(first_line) = lines.first() else {
            return "".into();
        };

        let ident_level = first_line.find(|ch| ch != ' ').unwrap();

        lines
            .into_iter()
            .map(|line| {
                let slice_start = line.find(|ch| ch != ' ').unwrap_or(0).min(ident_level);

                &line[slice_start..]
            })
            .intersperse("\n")
            .collect()
    }

    fn reindent(self, indentation: usize) -> String {
        let indentation_str = " ".repeat(indentation);
        self.remove_indent()
            .lines()
            .map(|line| format!("{}{}", indentation_str, line))
            .intersperse("\n".into())
            .collect()
    }
}

impl Indentable for String {
    fn remove_indent(self) -> String {
        self.as_str().remove_indent()
    }

    fn reindent(self, indentation: usize) -> String {
        self.as_str().reindent(indentation)
    }
}

#[cfg(test)]
mod test {
    mod remove_indent {
        use crate::code_utils::Indentable;

        #[test]
        fn empty_input() {
            assert_eq!("".remove_indent(), "");
        }

        #[test]
        fn unindented_input() {
            let input = r#"lol
            kek
                chebureck"#;

            assert_eq!(input.remove_indent(), input);
        }

        #[test]
        fn indented_input() {
            let input = r#"
            lol
                    kek
                chebureck
            "#;

            let expected_result = "lol\n        kek\n    chebureck";

            assert_eq!(input.remove_indent(), expected_result);
        }
    }

    mod reindent {
        use crate::code_utils::Indentable;

        #[test]
        fn empty_input() {
            assert_eq!("".reindent(4), "");
        }

        #[test]
        fn multiple_lines() {
            assert_eq!(
                "  lol\n kek\n    chebureck".reindent(1),
                " lol\n kek\n   chebureck"
            );
        }
    }
}
