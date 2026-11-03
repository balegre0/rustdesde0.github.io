// 1. Pide un closure que solo lee (Fn): se puede llamar muchas veces.
fn dos_veces<F: Fn()>(f: F) {
    f();
    f();
}

// 2. Pide un closure que modifica lo capturado (FnMut): necesita `mut`.
fn tres_veces<F: FnMut()>(mut f: F) {
    f();
    f();
    f();
}

// 3. Pide un closure que se puede llamar UNA vez (FnOnce).
fn una_vez<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn main() {
    let nombre = String::from("Noa");
    dos_veces(|| println!("hola, {nombre}"));

    let mut total = 0;
    tres_veces(|| total += 5);
    println!("total = {total}");

    let regalo = String::from("🎁 caja");
    // Este closure entrega (mueve hacia fuera) `regalo`: solo puede ocurrir una vez.
    let entregado = una_vez(move || regalo);
    println!("entregado: {entregado}");
}
