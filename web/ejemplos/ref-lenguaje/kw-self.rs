mod raiz {
    pub fn nombre() -> &'static str { "raiz" }
    pub mod hijo {
        pub fn nombre() -> &'static str { "hijo" }
        pub fn probar() {
            // 2. `self::` empieza en el módulo actual.
            println!("{}", self::nombre());
            println!("{}", super::nombre());
        }
    }
}

struct Contador { n: u32 }

impl Contador {
    // 1. `self` como receptor: `&mut self` = `self: &mut Self`.
    fn sumar(&mut self) -> &mut Self { self.n += 1; self }
    fn ver(&self) -> u32 { self.n }
    fn consumir(self) -> u32 { self.n * 10 }
}

fn main() {
    let mut c = Contador { n: 0 };
    c.sumar().sumar();
    println!("{}", c.ver());
    println!("{}", c.consumir());
    raiz::hijo::probar();
    // 3. `use ruta::{self, …}` importa también el propio módulo.
    use std::collections::{self, HashMap};
    let _m: HashMap<u8, u8> = collections::HashMap::new();
}
