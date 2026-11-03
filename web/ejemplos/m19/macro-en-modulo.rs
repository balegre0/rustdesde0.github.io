mod utilidades {
    // 1. Definida dentro de un módulo...
    macro_rules! grita {
        ($s:expr) => { format!("¡{}!", $s.to_uppercase()) };
    }
    // 2. ...y reexportada como un item normal con `use`
    pub(crate) use grita;
}

mod saludos {
    // 3. Ahora se importa por ruta, como una función
    use crate::utilidades::grita;
    pub fn hola() -> String { grita!("hola, señora") }
}

fn main() {
    println!("{}", saludos::hola());
    println!("{}", crate::utilidades::grita!("adiós"));
}
