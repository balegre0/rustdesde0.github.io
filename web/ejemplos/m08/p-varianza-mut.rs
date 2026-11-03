fn asignar<'a>(destino: &mut &'a str, valor: &'a str) {
    *destino = valor;
}

fn main() {
    let mut fijo: &'static str = "para siempre";
    {
        let corta = String::from("corta");
        asignar(&mut fijo, &corta);
    }
    println!("{fijo}");
}
