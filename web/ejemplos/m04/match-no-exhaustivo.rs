fn main() {
    let dado: u8 = 4;
    let texto = match dado {
        1 => "uno",
        2..=5 => "entre dos y cinco",
        6 => "seis",
    };
    println!("{texto}");
}
