// `&mut T` es INVARIANTE en T: no se puede «acortar» el 'static de dentro.
fn meter<'a>(destino: &mut &'a str, valor: &'a str) {
    *destino = valor;
}

fn main() {
    let mut s: &'static str = "para siempre";
    {
        let local = String::from("temporal");
        // Si se permitiera, `s` (que promete 'static) acabaría apuntando a `local`.
        meter(&mut s, &local);
    }
    println!("{s}");
}
