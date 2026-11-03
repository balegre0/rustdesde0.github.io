fn main() {
    let edad = "42";
    let edad: u32 = edad.parse().unwrap();
    let edad = edad + 1;
    println!("El año que viene tendrás {edad}");
}
