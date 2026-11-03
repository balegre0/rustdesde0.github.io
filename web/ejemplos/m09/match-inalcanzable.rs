fn main() {
    let n = 7;
    let texto = match n {
        _ => "cualquier cosa",
        7 => "siete",
    };
    println!("{texto}");
}
