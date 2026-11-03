fn main() {
    let literal: &str = "hola";
    let mut propio: String = String::from(literal);
    propio.push_str(", mundo");
    let vista: &str = &propio[..4];
    println!("{literal} | {propio} | {vista}");
}
