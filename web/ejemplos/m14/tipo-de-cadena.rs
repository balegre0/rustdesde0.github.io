fn main() {
    let v = vec![1, 2, 3];
    // Truco: pedir que la cadena sea `()` para que el compilador nos diga su tipo real.
    let () = v.iter().filter(|x| **x > 1).map(|x| x * 2);
}
