macro_rules! max_de {
    ($x:expr $(,)?) => { $x };
    ($x:expr, $($resto:expr),+ $(,)?) => {
        ::std::cmp::max($x, max_de!($($resto),+))
    };
}
