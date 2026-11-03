fn main() {
    let nada: Option<u32> = None;
    println!("antes");
    let n = nada.expect("se esperaba un número");
    println!("{n}");
}
