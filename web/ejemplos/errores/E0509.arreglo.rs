struct Registro {
    texto: String,
}

impl Drop for Registro {
    fn drop(&mut self) {
        println!("cerrando registro");
    }
}

fn main() {
    let mut r = Registro { texto: String::from("datos") };
    let texto = std::mem::take(&mut r.texto); // deja un String vacío en su sitio
    println!("{texto}");
}
