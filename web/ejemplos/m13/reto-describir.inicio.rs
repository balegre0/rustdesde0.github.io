trait Animal {
    fn nombre(&self) -> String;
    fn sonido(&self) -> String;

    // Escribe aquí un método por defecto `presentarse` que devuelva
    // "<nombre> dice <sonido>", usando los otros dos métodos.
}

struct Perro;
struct Vaca;

impl Animal for Perro {
    fn nombre(&self) -> String {
        String::from("El perro")
    }
    fn sonido(&self) -> String {
        String::from("guau")
    }
}

// Implementa Animal para Vaca: se llama "La vaca" y dice "muu".

fn main() {
    println!("{}", Perro.nombre());
    // Cuando esté hecho, cambia la línea de arriba por estas dos:
    // println!("{}", Perro.presentarse());
    // println!("{}", Vaca.presentarse());
}
