// ¿En qué orden se destruyen las cosas?
struct Ruido(&'static str);

impl Drop for Ruido {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

struct Par {
    _primero: Ruido,
    _segundo: Ruido,
}

fn main() {
    let _a = Ruido("a (local 1)");
    let _b = Ruido("b (local 2)");
    let _p = Par { _primero: Ruido("campo 1"), _segundo: Ruido("campo 2") };
    let _ = Ruido("temporal con _"); // `_` no enlaza: se destruye YA
    println!("fin de main");
}
