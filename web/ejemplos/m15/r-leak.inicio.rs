// Devuelve un &'static str construido en ejecución, a costa de no liberarlo nunca.
// Pista: format!, luego .into_boxed_str() y Box::leak.
fn nombre_global(n: u32) -> &'static str {
    let _ = n;
    todo!()
}

fn main() {
    let a = nombre_global(7);
    let b = nombre_global(42);
    println!("{a} y {b}");
}
