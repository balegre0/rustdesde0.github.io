fn main() {
    let nota: u8 = 150;
    let t = match nota {
        0..=100 => "normal",
        101..=199 => "alta",
    };
    println!("{t}");
}
