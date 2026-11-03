use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // 1. escribir un archivo entero (lo crea o lo sobrescribe)
    fs::write("notas.txt", "primera línea\nsegunda línea\n")?;
    // 2. leerlo entero como texto
    let texto = fs::read_to_string("notas.txt")?;
    println!("{texto:?}");
    // 3. carpetas
    fs::create_dir_all("datos/2024")?;
    fs::write("datos/2024/a.txt", "A")?;
    fs::copy("notas.txt", "datos/copia.txt")?;
    let mut nombres: Vec<String> = fs::read_dir("datos")?
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    nombres.sort();
    println!("{nombres:?}");
    // 4. metadatos, renombrar y borrar
    println!("{} bytes", fs::metadata("notas.txt")?.len());
    fs::rename("notas.txt", "viejas.txt")?;
    fs::remove_file("viejas.txt")?;
    fs::remove_dir_all("datos")?;
    // 5. los errores son valores: io::Error
    match fs::read_to_string("no-existe.txt") {
        Ok(_) => println!("¿?"),
        Err(e) => println!("error: {:?}", e.kind()),
    }
    Ok(())
}
