// Completa el constructor `Dado::new(caras)` y la función asociada
// `Dado::clasico()`, que crea un dado de 6 caras usando `Self::new`.
// Después, descomenta las líneas de main.

struct Dado {
    caras: u32,
}

impl Dado {
    // escribe aquí `new` y `clasico`

    fn describir(&self) -> String {
        format!("dado de {} caras", self.caras)
    }
}

fn main() {
    // let d20 = Dado::new(20);
    // let d6 = Dado::clasico();
    // println!("{}", d20.describir());
    // println!("{}", d6.describir());
}
