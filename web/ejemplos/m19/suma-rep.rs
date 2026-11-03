// 1. $( ... ),* : cero o más expresiones separadas por comas
//    $(,)?      : una coma final opcional
macro_rules! suma {
    ($($x:expr),* $(,)?) => {
        0 $(+ $x)*
    };
}

fn main() {
    println!("{}", suma!());
    println!("{}", suma!(5));
    println!("{}", suma!(1, 2, 3));
    println!("{}", suma!(10, 20, 30,));
}
