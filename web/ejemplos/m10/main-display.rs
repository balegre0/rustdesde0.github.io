use std::error::Error;
use std::process::ExitCode;

fn ejecutar() -> Result<u32, Box<dyn Error>> {
    let n: u32 = "cuarenta".parse()?;
    Ok(n)
}

// Patrón para mostrar el error con Display (bonito) y elegir el código de salida.
fn main() -> ExitCode {
    match ejecutar() {
        Ok(n) => {
            println!("resultado: {n}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}
