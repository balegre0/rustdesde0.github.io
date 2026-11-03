fn main() {
    let grados = "21";
    let grados = grados.parse::<i32>().unwrap();
    let grados = grados + 1;
    println!("mañana: {grados}");
}
