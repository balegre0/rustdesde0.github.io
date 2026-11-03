fn main() {
    let dado: u8 = 4;
    let texto = match dado {
        1 => "uno",
        2..=5 => "entre dos y cinco",
        6 => "seis",
        _ => "no es un dado", // cubre 0 y 7..=255
    };
    println!("{texto}");
}
