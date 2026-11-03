// Un programa de Windows sin ventana de consola (por ejemplo, un programa con ventanas).
// cfg_attr: solo en release, para seguir viendo println! al depurar.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    println!("en debug verás esto; en release no hay consola");
}
