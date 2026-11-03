fn consumir(s: String) {
    println!("me quedo con {s}");
}

fn main() {
    let nombre = String::from("Ana");
    let r = &nombre;
    consumir(nombre);
    println!("{r}");
}
