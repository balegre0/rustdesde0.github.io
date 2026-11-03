struct Vista {
    texto: &'a str,
}

fn main() {
    let v = Vista { texto: "hola" };
    println!("{}", v.texto);
}
