use std::fmt::Display;

// `impl Display` en un argumento = «algún tipo que implemente Display».
fn entre_corchetes(x: impl Display) -> String {
    format!("[{x}]")
}

fn main() {
    let a = entre_corchetes(5);
    let b = entre_corchetes("cinco");
    let c = entre_corchetes(a.clone() + &b);
    println!("{c}");
}
