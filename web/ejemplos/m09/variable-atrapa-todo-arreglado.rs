fn main() {
    let objetivo = 5;
    let tirada = 2;
    match tirada {
        t if t == objetivo => println!("¡acertaste! ({t})"),
        _ => println!("fallaste"),
    }
}
