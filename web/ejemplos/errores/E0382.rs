fn main() {
    let saludo = String::from("hola");
    let otro = saludo; // la propiedad se mueve a `otro`
    println!("{saludo}");
    println!("{otro}");
}
