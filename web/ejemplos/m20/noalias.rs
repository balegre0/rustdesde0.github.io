// Dos préstamos exclusivos nunca apuntan al mismo sitio.
fn sumar_en(destino: &mut i32, origen: &mut i32) {
    *destino += *origen;
    *origen = 0;
}

fn main() {
    let mut a = 5;
    sumar_en(&mut a, &mut a);
    println!("{a}");
}
