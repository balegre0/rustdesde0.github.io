fn main() {
    let nombre = String::from("Ana");
    let apellido = String::from("Gil");
    let completo = format!("{nombre} {apellido}");
    println!("{completo} (y siguen vivos: {nombre}, {apellido})");
}
