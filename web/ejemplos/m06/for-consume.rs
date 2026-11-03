fn main() {
    let nombres = vec![String::from("Ana"), String::from("Luis")];
    for n in nombres {
        println!("{n}");
    }
    println!("había {} nombres", nombres.len());
}
