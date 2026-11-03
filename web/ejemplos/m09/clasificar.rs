fn clasificar(edad: u32, socio: bool) -> &'static str {
    match (edad, socio) {
        (0..=11, _) => "infantil",
        (65.., _) => "senior",
        (_, true) => "socio",
        _ => "general",
    }
}

fn main() {
    println!("{}", clasificar(8, true));
    println!("{}", clasificar(70, false));
    println!("{}", clasificar(30, true));
    println!("{}", clasificar(30, false));
}
