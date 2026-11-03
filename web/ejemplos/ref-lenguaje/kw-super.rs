fn saludo() -> &'static str { "hola desde la raíz" }

mod a {
    pub fn nombre() -> &'static str { "a" }
    pub mod b {
        pub fn probar() {
            // `super` = el módulo padre; se puede repetir.
            println!("{}", super::nombre());
            println!("{}", super::super::saludo());
        }
    }
}

fn main() {
    a::b::probar();
}
