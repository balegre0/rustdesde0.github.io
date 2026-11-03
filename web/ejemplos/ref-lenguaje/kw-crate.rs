mod tienda {
    pub mod caja {
        pub fn cobrar() -> u32 {
            // 1. `crate::` empieza la ruta en la raíz del crate.
            crate::impuestos::con_iva(100)
        }
    }
    // 2. `pub(crate)`: visible en todo este crate, pero no fuera de él.
    pub(crate) fn nombre() -> &'static str {
        "La Tiendita"
    }
}

mod impuestos {
    pub fn con_iva(base: u32) -> u32 {
        base * 121 / 100
    }
}

fn main() {
    println!("{}", tienda::nombre());
    println!("{}", crate::tienda::caja::cobrar());
}
