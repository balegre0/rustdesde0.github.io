fn main() {
    let n: u8 = 200;
    // Un u8 va de 0 a 255: falta cubrir 201..=255.
    let t = match n {
        0..=100 => "bajo",
        101..=200 => "medio",
    };
    println!("{t}");
}
