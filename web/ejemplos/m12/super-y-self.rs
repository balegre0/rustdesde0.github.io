mod tienda {
    const IVA: f64 = 0.21;

    pub mod caja {
        pub fn total(precio: f64) -> f64 {
            // super:: sube un nivel: al módulo `tienda`.
            precio * (1.0 + super::IVA)
        }

        pub fn ticket(precio: f64) -> String {
            // self:: es el módulo actual (aquí, `caja`).
            format!("{:.2} €", self::total(precio))
        }
    }
}

fn main() {
    println!("{}", tienda::caja::ticket(10.0));
}
