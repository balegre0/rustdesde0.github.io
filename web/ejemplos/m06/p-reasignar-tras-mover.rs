fn main() {
    let mut s = String::from("viejo");
    let t = s;
    s = String::from("nuevo");
    println!("{s} {t}");
}
