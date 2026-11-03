fn main() {
    // 1. Recorrer un rango.
    for i in 0..3 {
        print!("{i} ");
    }
    println!();
    // 2. Recorrer una colección prestada: `&v` da referencias.
    let v = vec!["a", "b", "c"];
    for (i, letra) in v.iter().enumerate() {
        print!("{i}:{letra} ");
    }
    println!();
    // 3. Patrón en el `for`: desestructurar pares.
    let pares = [(1, 'x'), (2, 'y')];
    for (n, c) in pares {
        print!("{n}{c} ");
    }
    println!();
    // 4. `for<'a>`: límite de orden superior (vale para cualquier tiempo de vida).
    fn llamar<F>(f: F) -> usize where F: for<'a> Fn(&'a str) -> usize { f("hola") }
    println!("{}", llamar(|s| s.len()));
}
