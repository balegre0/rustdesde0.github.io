fn llamar_dos_veces<F: Fn()>(f: F) {
    f();
    f();
}

fn main() {
    let nombre = String::from("Ana");
    let despedir = move || {
        let prestado = &nombre; // solo lo mira: la closure es Fn
        println!("adiós, {prestado}");
    };
    llamar_dos_veces(despedir);
}
