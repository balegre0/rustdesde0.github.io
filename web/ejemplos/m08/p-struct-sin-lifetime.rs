struct Extracto {
    parte: &str,
}

fn main() {
    let e = Extracto { parte: "hola" };
    println!("{}", e.parte);
}
