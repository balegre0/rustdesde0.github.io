struct Extracto<'a> {
    parte: &'a str,
}

fn main() {
    let e;
    {
        let novela = String::from("Era una vez. Fin.");
        e = Extracto { parte: &novela[..11] };
    }
    println!("{}", e.parte);
}
