// Pedimos FnMut: ahora el closure puede modificar lo que captura.
fn llama<F: FnMut()>(mut f: F) {
    f();
}

fn main() {
    let mut pasos = 0;
    llama(|| pasos += 1);
    println!("{pasos}");
}
