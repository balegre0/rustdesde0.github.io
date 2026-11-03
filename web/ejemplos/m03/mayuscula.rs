fn main() {
    let c = 'ñ';
    println!("{c} U+{:X} {} {}", c as u32, c.is_alphabetic(), c.to_uppercase());
}
