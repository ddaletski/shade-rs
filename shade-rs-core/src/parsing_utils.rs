
#[macro_export]
macro_rules! parsing_error {
    ($tree:expr, $($fmt_args:expr),+) => {
        let msg = format!($($fmt_args),*);
        let span = $tree.span().unwrap();
        span.error(msg).emit();
        unreachable!(); // for unstructuring let to see that this function always panics
    };
}
