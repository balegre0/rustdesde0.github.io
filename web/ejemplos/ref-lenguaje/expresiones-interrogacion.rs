use std::num::ParseIntError;

// `?` devuelve el error en cuanto aparece; si no, desenvuelve el valor.
fn sumar_textos(a: &str, b: &str) -> Result<i32, ParseIntError> {
    Ok(a.parse::<i32>()? + b.parse::<i32>()?)
}

// También funciona con Option.
fn inicial_mayuscula(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

fn main() {
    println!("{:?}", sumar_textos("2", "40"));
    println!("{:?}", sumar_textos("2", "x"));
    println!("{:?} {:?}", inicial_mayuscula("rust"), inicial_mayuscula(""));
    // Rangos: a..b excluye b; a..=b lo incluye.
    let v: Vec<i32> = (1..4).collect();
    let w: Vec<i32> = (1..=4).collect();
    println!("{v:?} {w:?}");
    // Asignación con desestructuración y `_`.
    let (mut x, mut y) = (1, 2);
    (x, y) = (y, x);
    let z;
    (z, _) = (9, 8);
    println!("{x} {y} {z}");
}
