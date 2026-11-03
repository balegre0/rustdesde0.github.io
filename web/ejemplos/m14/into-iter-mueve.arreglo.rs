fn main() {
    let nombres = vec![String::from("Ana"), String::from("Óscar")];
    // `&nombres`: recorremos con préstamos, sin mover el vector.
    for n in &nombres {
        println!("{n}");
    }
    println!("había {} nombres", nombres.len());
}
