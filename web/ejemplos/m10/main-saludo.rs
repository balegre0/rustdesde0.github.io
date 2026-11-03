use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let veces: u32 = "3".parse()?;
    for _ in 0..veces {
        println!("hola");
    }
    Ok(())
}
