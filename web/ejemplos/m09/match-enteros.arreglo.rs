fn tipo_de_nota(n: u8) -> &'static str {
    match n {
        0..=4 => "suspenso",
        5..=6 => "aprobado",
        7..=8 => "notable",
        9..=10 => "sobresaliente",
        _ => "nota imposible",
    }
}

fn main() {
    println!("{}", tipo_de_nota(7));
    println!("{}", tipo_de_nota(42));
}
