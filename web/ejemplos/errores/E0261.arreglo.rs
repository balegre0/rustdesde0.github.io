struct Vista<'a> {
    texto: &'a str,
}

fn main() {
    let v = Vista { texto: "hola" };
    println!("{}", v.texto);
}
