fn primera_palabra(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    let frase = String::from("hola mundo");
    println!("[{}]", primera_palabra(&frase));
}
