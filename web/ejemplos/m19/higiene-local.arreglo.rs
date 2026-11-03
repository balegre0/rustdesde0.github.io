macro_rules! mas_uno {
    ($v:ident) => { $v + 1 };
}

fn main() {
    let y = 10;
    println!("{}", mas_uno!(y));
}
