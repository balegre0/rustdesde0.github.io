// Dos referencias de entrada y ninguna es &self: el compilador no adivina
// de cuál de las dos sale el resultado.
fn mas_larga(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("{}", mas_larga("uno", "tres"));
}
