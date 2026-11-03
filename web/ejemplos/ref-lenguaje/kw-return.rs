fn primer_negativo(v: &[i32]) -> Option<i32> {
    for &x in v {
        // 1. `return` sale de la función en cualquier punto.
        if x < 0 { return Some(x); }
    }
    None // 2. Al final se suele omitir `return`.
}

fn main() {
    println!("{:?}", primer_negativo(&[3, -2, -5]));
    println!("{:?}", primer_negativo(&[1, 2]));
    // 3. Dentro de una closure, `return` sale de la closure, no de `main`.
    let f = |x: i32| { if x > 0 { return "positivo"; } "no positivo" };
    println!("{}", f(4));
    println!("fin");
}
