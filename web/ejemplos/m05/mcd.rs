fn mcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { mcd(b, a % b) }
}

fn simplificar(num: u32, den: u32) {
    let m = mcd(num, den);
    println!("{num}/{den} = {}/{}", num / m, den / m);
}

fn main() {
    println!("mcd(48, 36) = {}", mcd(48, 36));
    println!("mcd(17, 5) = {}", mcd(17, 5));
    simplificar(12, 18);
    simplificar(100, 25);
}
