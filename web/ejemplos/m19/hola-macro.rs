// 1. Una macro con dos reglas (brazos)
macro_rules! saluda {
    () => {
        println!("¡Hola, mundo!")
    };
    ($nombre:expr) => {
        println!("¡Hola, {}!", $nombre)
    };
}

fn main() {
    // 2. Se invoca con ! y puede usar (), [] o {}
    saluda!();
    saluda!("Lucía");
    saluda![String::from("Íñigo")];
    saluda! { "Mar" }
}
