unsafe extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    // SAFETY: abs de la biblioteca estándar de C acepta cualquier i32 salvo el mínimo,
    // y -3 no lo es.
    let r = unsafe { abs(-3) };
    println!("{r}");
}
