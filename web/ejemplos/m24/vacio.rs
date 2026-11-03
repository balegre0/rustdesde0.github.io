use std::convert::Infallible;

// Infallible no tiene ningún valor: es el conjunto vacío.
// Una función que RECIBE un Infallible nunca puede llamarse, así que puede «devolver lo que sea».
fn absurdo<T>(x: Infallible) -> T {
    match x {}
}

// Un Result cuyo error es imposible: sacarlo nunca falla.
fn siempre_ok(r: Result<u32, Infallible>) -> u32 {
    match r {
        Ok(n) => n,
        Err(e) => absurdo(e),
    }
}

fn main() {
    println!("bytes de Infallible: {}", size_of::<Infallible>());
    println!("bytes de Result<u32, Infallible>: {}", size_of::<Result<u32, Infallible>>());
    println!("bytes de u32: {}", size_of::<u32>());
    let n: Result<u32, Infallible> = "42".parse::<u32>().map_err(|_| unreachable!());
    println!("{}", siempre_ok(n));
}
