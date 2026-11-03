fn main() {
    let edad = 20;
    let tarifa: u32 = if edad < 18 { 5 } else { 8 };
    println!("{tarifa}");
}
