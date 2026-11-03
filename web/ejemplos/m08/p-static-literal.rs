fn saludo() -> &'static str {
    "hola"
}

fn main() {
    let s;
    {
        s = saludo();
    }
    println!("{s}");
}
