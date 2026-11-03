// Rust libera el heap automáticamente cuando el dueño se acaba.
// Para verlo, este tipo avisa justo antes de liberarse.
struct Avisador {
    nombre: String,
}

impl Drop for Avisador {
    fn drop(&mut self) {
        println!("  -> libero el heap de «{}»", self.nombre);
    }
}

fn main() {
    // 1. Un dueño en main
    let _uno = Avisador { nombre: String::from("uno") };

    // 2. Un dueño dentro de un bloque { }
    {
        let _dos = Avisador { nombre: String::from("dos") };
        println!("dentro del bloque");
    } // aquí se acaba _dos

    // 3. El dueño cambia: _tres pasa a llamarse _otro (se «mueve»)
    let _tres = Avisador { nombre: String::from("tres") };
    let _otro = _tres;
    println!("fin de main");
} // aquí se acaban _otro y _uno, en orden inverso
