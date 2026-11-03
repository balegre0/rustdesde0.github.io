fn main() {
    let edades = [30, 41];
    match edades.get(1) {
        Some(edad) => println!("{}", edad + 10),
        None => println!("no hay edad"),
    }
}
