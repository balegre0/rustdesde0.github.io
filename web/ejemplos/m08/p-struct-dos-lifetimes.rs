struct Par<'a, 'b> {
    clave: &'a str,
    valor: &'b str,
}

fn clave<'a, 'b>(p: &Par<'a, 'b>) -> &'a str {
    p.clave
}

fn main() {
    let k = String::from("color");
    let c;
    {
        let v = String::from("rojo");
        let p = Par { clave: &k, valor: &v };
        println!("{} = {}", p.clave, p.valor);
        c = clave(&p);
    }
    println!("{c}");
}
