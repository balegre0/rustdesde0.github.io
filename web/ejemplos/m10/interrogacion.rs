use std::num::ParseIntError;

// 1. Con match: correcto, pero mucho ruido.
fn doble_a_mano(texto: &str) -> Result<i32, ParseIntError> {
    let n: i32 = match texto.parse::<i32>() {
        Ok(valor) => valor,
        Err(e) => return Err(e), // retorno temprano con el error
    };
    Ok(n * 2)
}

// 2. Con ?: exactamente lo mismo, en una línea.
fn doble(texto: &str) -> Result<i32, ParseIntError> {
    let n: i32 = texto.parse()?;
    Ok(n * 2)
}

// 3. ? también funciona con Option (dentro de funciones que devuelven Option).
fn inicial_mayuscula(texto: &str) -> Option<char> {
    let c = texto.chars().next()?;
    c.to_uppercase().next()
}

// 4. Se pueden encadenar: cada ? es un posible punto de salida.
fn media(a: &str, b: &str, c: &str) -> Result<i32, ParseIntError> {
    Ok((a.parse::<i32>()? + b.parse::<i32>()? + c.parse::<i32>()?) / 3)
}

fn main() {
    println!("{:?} {:?}", doble_a_mano("21"), doble_a_mano("x"));
    println!("{:?} {:?}", doble("21"), doble("x"));
    println!("{:?} {:?}", inicial_mayuscula("ñu"), inicial_mayuscula(""));
    println!("{:?} {:?}", media("3", "6", "9"), media("3", "seis", "9"));
}
