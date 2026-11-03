// Escribe A MANO lo que generaría #[derive(Default, PartialEq)] para Config.
// (Pista: Default da 0, false y String vacío; PartialEq compara campo a campo.)
#[derive(Debug)]
struct Config {
    hilos: u32,
    verboso: bool,
    nombre: String,
}

// impl Default for Config { ... }
// impl PartialEq for Config { ... }

fn main() {
    let c = Config { hilos: 0, verboso: false, nombre: String::new() };
    println!("{:?}", c);
    // println!("{:?}", Config::default());
    // println!("{}", c == Config::default());
}
