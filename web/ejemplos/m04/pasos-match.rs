fn main() {
    let nota = 7;
    let texto = match nota {
        0..=4 => "suspenso",
        5..=6 => "aprobado",
        7..=8 => "notable",
        _ => "sobresaliente",
    };
    println!("Un {nota} es un {texto}");
}
