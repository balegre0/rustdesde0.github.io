fn para_siempre(x: &i32) -> Box<dyn Fn() + 'static> {
    let copia = *x; // la closure se lleva un i32 propio, no una referencia
    Box::new(move || println!("{copia}"))
}

fn main() {
    let n = 5;
    let f = para_siempre(&n);
    f();
}
