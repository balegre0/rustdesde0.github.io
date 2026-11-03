fn contar_a(s: &String) -> usize {
    let mut n = 0;
    for c in s.chars() {
        if c == 'a' {
            n += 1;
        }
    }
    n
}

fn main() {
    let palabra = String::from("banana");
    let a = contar_a(&palabra);
    let b = contar_a(&palabra);
    println!("{palabra}: {a} + {b} = {}", a + b);
}
