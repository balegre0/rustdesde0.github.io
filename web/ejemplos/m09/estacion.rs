fn estacion(mes: u32) -> &'static str {
    match mes {
        3..=5 => "primavera",
        6..=8 => "verano",
        9..=11 => "otoño",
        12 | 1 | 2 => "invierno",
        _ => "mes inválido",
    }
}

fn main() {
    for mes in [1, 4, 7, 10, 12, 13] {
        println!("{mes}: {}", estacion(mes));
    }
}
