struct Extracto<'a> {
    parte: &'a str,
}

fn main() {
    let e = Extracto { parte: "hola" };
    println!("{}", e.parte);
}
