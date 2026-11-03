use std::mem;

struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("adiós, {}", self.0);
    }
}

fn main() {
    // 1. mem::forget: se olvida del valor sin ejecutar su Drop. No es unsafe.
    let olvidado = Ruidoso("olvidado");
    mem::forget(olvidado);

    // 2. Box::leak: suelta la Box y te da una referencia que vale para siempre.
    let config: &'static mut String = Box::leak(Box::new(String::from("modo=rápido")));
    config.push_str(";nivel=3");
    println!("config = {config}");

    let _normal = Ruidoso("normal");
    println!("fin de main");
    // 3. Solo «normal» dice adiós; «olvidado» nunca lo hará.
}
