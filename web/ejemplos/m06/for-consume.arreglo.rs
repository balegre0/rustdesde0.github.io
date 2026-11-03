fn main() {
    let nombres = vec![String::from("Ana"), String::from("Luis")];
    // `&nombres`: recorremos prestado; el Vec sigue siendo el dueño
    for n in &nombres {
        println!("{n}");
    }
    println!("había {} nombres", nombres.len());
}
