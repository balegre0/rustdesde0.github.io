fn main() {
    let objetivo = 5;
    let tirada = 2;
    match tirada {
        objetivo => println!("¡acertaste! ({objetivo})"),
        _ => println!("fallaste"),
    }
}
