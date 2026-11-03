fn main() {
    let nombre = "ana";
    match nombre.chars().next() {
        Some('a'..='m') => println!("primera mitad"),
        _ => println!("segunda mitad"),
    }
}
