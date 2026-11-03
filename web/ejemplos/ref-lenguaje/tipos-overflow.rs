fn main() {
    let x: u8 = 255;
    // En modo debug, desbordar un entero es un pánico; en release, da la vuelta.
    let y = x + std::hint::black_box(1);
    println!("{y}");
}
