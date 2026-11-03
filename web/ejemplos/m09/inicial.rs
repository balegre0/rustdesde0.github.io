fn inicial(texto: &str) -> Option<char> {
    texto.chars().next()
}

fn main() {
    println!("{:?}", inicial("Ñandú"));
    println!("{:?}", inicial(""));
}
