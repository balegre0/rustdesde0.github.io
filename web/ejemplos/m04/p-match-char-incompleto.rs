fn main() {
    let c = 'x';
    let tipo = match c {
        'a'..='z' => "minúscula",
        'A'..='Z' => "mayúscula",
    };
    println!("{tipo}");
}
