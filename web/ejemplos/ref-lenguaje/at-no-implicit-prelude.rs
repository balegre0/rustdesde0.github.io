#[no_implicit_prelude]
mod sin_preludio {
    // Aquí no existen Vec, String, Option… ni println! por su nombre corto.
    pub fn saludar() {
        ::std::println!("hola desde un módulo sin preludio");
        let v: ::std::vec::Vec<u8> = ::std::vec::Vec::new();
        ::std::println!("{}", v.len());
    }
}

fn main() {
    sin_preludio::saludar();
}
