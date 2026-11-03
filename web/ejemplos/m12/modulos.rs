// Un módulo agrupa código con nombre propio. Puede estar dentro de otro.
mod cocina {
    pub fn preparar() -> String {
        let base = horno::hornear("pan");
        format!("{base} con {}", salsas::la_de_la_casa())
    }

    mod horno {
        pub fn hornear(que: &str) -> String {
            format!("{que} horneado")
        }
    }

    pub mod salsas {
        pub fn la_de_la_casa() -> &'static str {
            "alioli"
        }
    }
}

fn main() {
    // Ruta absoluta (desde la raíz del crate) y ruta relativa: llevan al mismo sitio.
    println!("{}", crate::cocina::preparar());
    println!("{}", cocina::salsas::la_de_la_casa());
}
