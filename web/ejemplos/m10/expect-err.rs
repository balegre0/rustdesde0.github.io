fn main() {
    let edad: u8 = "-3".parse().expect("la edad debe ser un número entre 0 y 255");
    println!("{edad}");
}
