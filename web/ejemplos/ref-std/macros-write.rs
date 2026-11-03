use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // en un String (fmt::Write)
    let mut s = String::new();
    write!(s, "{}", 1)?;
    writeln!(s, " y {}", 2)?;
    write!(s, "fin")?;
    println!("{s:?}");
    // en un Vec<u8> o en stdout (io::Write)
    let mut bytes: Vec<u8> = Vec::new();
    writeln!(bytes, "ñ={}", 'ñ')?;
    println!("{bytes:?}");
    writeln!(std::io::stdout(), "directo a la consola")?;
    Ok(())
}
