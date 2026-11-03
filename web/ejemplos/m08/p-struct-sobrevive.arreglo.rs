struct Extracto<'a> {
    parte: &'a str,
}

fn main() {
    let novela = String::from("Era una vez. Fin.");
    let e = Extracto { parte: &novela[..11] };
    println!("{}", e.parte);
}
