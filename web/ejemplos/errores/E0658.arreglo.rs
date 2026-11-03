fn sumar_uno(t: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(t.parse::<i32>()? + 1)
}

fn main() {
    println!("{:?}", sumar_uno("5"));
}
