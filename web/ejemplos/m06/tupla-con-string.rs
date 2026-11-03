fn main() {
    let ficha = (7, String::from("Ana"));
    let copia = ficha;
    println!("{}", copia.1);
    println!("{}", ficha.1);
}
