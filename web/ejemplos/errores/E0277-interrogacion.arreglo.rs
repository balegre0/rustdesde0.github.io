fn leer() -> Result<i32, std::num::ParseIntError> {
    let n: i32 = "7".parse()?;
    Ok(n)
}

fn main() {
    println!("{:?}", leer());
}
