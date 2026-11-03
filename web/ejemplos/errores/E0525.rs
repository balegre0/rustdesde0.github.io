fn llamar_dos_veces<F: Fn()>(f: F) {
    f();
    f();
}

fn main() {
    let nombre = String::from("Ana");
    let despedir = move || {
        let dueno = nombre;
        println!("adiós, {dueno}");
    };
    llamar_dos_veces(despedir);
}
