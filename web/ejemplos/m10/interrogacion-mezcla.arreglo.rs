fn primera_cifra(texto: &str) -> Option<u32> {
    let n: u32 = texto.parse().ok()?;
    n.to_string().chars().next()?.to_digit(10)
}

fn main() {
    println!("{:?}", primera_cifra("731"));
    println!("{:?}", primera_cifra("hola"));
}
