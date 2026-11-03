fn main() {
    let precio = "12".parse::<u32>();
    let con_iva = precio * 121 / 100;
    println!("{con_iva}");
}
