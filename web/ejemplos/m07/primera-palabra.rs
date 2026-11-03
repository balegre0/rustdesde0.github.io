fn primera_palabra(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    let frase = String::from("año nuevo feliz");
    let p = primera_palabra(&frase);
    println!("«{p}» empieza en el byte 0 y mide {} bytes", p.len());
    println!("con un literal: «{}»", primera_palabra("hola"));
}
