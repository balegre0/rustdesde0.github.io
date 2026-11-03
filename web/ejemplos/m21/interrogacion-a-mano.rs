use std::num::ParseIntError;

// 1. Con el operador `?`
fn doble(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

// 2. Lo mismo, escrito como lo deja el lowering (con match en vez de Try::branch)
fn doble_a_mano(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = match s.parse::<i32>() {
        Ok(val) => val,
        Err(residuo) => return Err(From::from(residuo)),
    };
    Ok(n * 2)
}

fn main() {
    println!("{:?} {:?}", doble("21"), doble_a_mano("21"));
    println!("{:?}", doble("veintiuno"));
    println!("{:?}", doble_a_mano("veintiuno"));
}
