fn main() {
    let palabra = "año";
    let al_reves: String = palabra.chars().rev().collect();
    println!("{al_reves}");
    println!("{}", al_reves.len());
}
