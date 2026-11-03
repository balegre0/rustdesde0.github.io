fn guardar_para_luego<F: Fn() + 'static>(f: F) -> Box<dyn Fn()> {
    Box::new(f)
}

fn preparar(nombre: &str) -> Box<dyn Fn()> {
    let propio = nombre.to_string(); // un String propio: no depende de nadie
    guardar_para_luego(move || println!("hola, {propio}"))
}

fn main() {
    let saludo = preparar("Ana");
    saludo();
}
