use std::error::Error;

fn leer(texto: &str) -> Result<f64, Box<dyn Error>> {
    let (a, b) = texto.split_once('/').ok_or("falta la barra")?;
    let num: i32 = a.parse()?;
    let den: f64 = b.parse()?;
    Ok(num as f64 / den)
}

fn main() {
    for t in ["3/4", "3", "x/4", "3/y"] {
        match leer(t) {
            Ok(v) => println!("{t}: {v}"),
            Err(e) => println!("{t}: error: {e}"),
        }
    }
}
