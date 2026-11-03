use std::num::ParseIntError;

fn sumar(textos: &[&str]) -> Result<i64, ParseIntError> {
    textos
        .iter()
        .map(|t| t.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()
        .map(|v| v.iter().sum())
}

fn main() {
    println!("{:?}", sumar(&["10", "20", "12"]));
    println!("{:?}", sumar(&["10", "veinte", "12"]));
}
