// https://stackoverflow.com/a/65618681

#[macro_export]
macro_rules! assert_with_pattern {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
        }
    }
}
