struct Registro {
    texto: String,
}

impl Drop for Registro {
    fn drop(&mut self) {
        println!("cerrando registro");
    }
}

fn main() {
    let r = Registro { texto: String::from("datos") };
    let Registro { texto } = r;
    println!("{texto}");
}
