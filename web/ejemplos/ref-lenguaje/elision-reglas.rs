// Regla 2: una sola referencia de entrada -> la salida toma su tiempo de vida.
fn primera_palabra(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}
// Es exactamente lo mismo que:
fn primera_palabra_explicita<'a>(s: &'a str) -> &'a str {
    s.split(' ').next().unwrap_or("")
}

struct Libro { titulo: String }

impl Libro {
    // Regla 3: con &self, la salida toma el tiempo de vida de self,
    // aunque haya otras referencias de entrada.
    fn titulo_o<'b>(&self, _otro: &'b str) -> &str {
        &self.titulo
    }
}

// `'_`: «aquí hay un tiempo de vida, dedúcelo».
struct Vista<'a> { texto: &'a str }
fn vista(s: &str) -> Vista<'_> { Vista { texto: s } }

fn main() {
    let frase = String::from("hola mundo");
    println!("{}", primera_palabra(&frase));
    println!("{}", primera_palabra_explicita(&frase));
    let l = Libro { titulo: "Rust".into() };
    println!("{}", l.titulo_o("otro"));
    println!("{}", vista(&frase).texto);
}
