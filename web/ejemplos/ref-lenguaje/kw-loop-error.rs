fn main() {
    let mut n = 0;
    // Todos los `break` de un mismo `loop` deben dar el mismo tipo.
    let r = loop {
        n += 1;
        if n == 1 { break 1; }
        if n == 2 { break "dos"; }
    };
    println!("{r}");
}
