fn doble(x: i32) -> i32 {
    x * 2
}

fn main() {
    // `doble` a secas es un «elemento función»: un tipo propio de tamaño cero.
    let f = doble;
    println!("elemento función: {} bytes", size_of_val(&f));
    // Convertido a puntero a función, guarda una dirección.
    let p: fn(i32) -> i32 = doble;
    println!("puntero a función: {} bytes (lo que una dirección)", size_of_val(&p));
    println!("{} {}", f(4), p(4));
}
