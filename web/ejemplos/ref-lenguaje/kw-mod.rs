// 1. Módulo en línea: agrupa elementos y crea un espacio de nombres.
mod geometria {
    pub mod formas {
        pub fn area_cuadrado(l: f64) -> f64 { l * l }
    }
    // Privado: solo visible dentro de `geometria` y sus hijos.
    fn ayuda() -> &'static str { "privada" }
    pub fn usar_ayuda() -> &'static str { ayuda() }
}

// 2. `mod nombre;` (sin llaves) cargaría el módulo desde `nombre.rs`
//    o `nombre/mod.rs`. En un ejemplo de un solo archivo usamos la forma en línea.

fn main() {
    println!("{}", geometria::formas::area_cuadrado(3.0));
    println!("{}", geometria::usar_ayuda());
}
