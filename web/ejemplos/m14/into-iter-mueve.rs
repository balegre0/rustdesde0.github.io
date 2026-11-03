fn main() {
    let nombres = vec![String::from("Ana"), String::from("Óscar")];
    for n in nombres {
        println!("{n}");
    }
    println!("había {} nombres", nombres.len());
}
