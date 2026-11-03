fn primera_palabra(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    let mut frase = String::from("hola mundo");
    let p = primera_palabra(&frase);
    frase.clear();
    println!("{p}");
}
