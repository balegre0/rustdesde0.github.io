use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: i32 = "12".parse()?;
    println!("{n}");
    Ok(())
}
