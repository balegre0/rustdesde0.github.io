fn guardar_para_luego<F: Fn() + 'static>(f: F) -> Box<dyn Fn()> {
    Box::new(f)
}

fn preparar(nombre: &str) -> Box<dyn Fn()> {
    guardar_para_luego(move || println!("hola, {nombre}"))
}

fn main() {
    let saludo = preparar("Ana");
    saludo();
}
