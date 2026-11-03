// Completa `sumar` en UNA expresión: convierte cada texto a i64 con map
// y recoge en Result<Vec<i64>, _> con collect; luego suma con .map(|v| v.iter().sum()).

use std::num::ParseIntError;

fn sumar(textos: &[&str]) -> Result<i64, ParseIntError> {
    todo!()
}

fn main() {
    println!("{:?}", sumar(&["10", "20", "12"]));
    println!("{:?}", sumar(&["10", "veinte", "12"]));
}
