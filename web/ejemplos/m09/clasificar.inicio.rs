// Completa `clasificar` con UN match sobre la tupla (edad, socio):
//   edad 0..=11              -> "infantil"
//   edad 65 o más            -> "senior"
//   socio == true (resto)    -> "socio"
//   cualquier otro caso      -> "general"

fn clasificar(edad: u32, socio: bool) -> &'static str {
    todo!()
}

fn main() {
    println!("{}", clasificar(8, true));
    println!("{}", clasificar(70, false));
    println!("{}", clasificar(30, true));
    println!("{}", clasificar(30, false));
}
