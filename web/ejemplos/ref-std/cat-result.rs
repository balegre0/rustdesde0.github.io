use std::num::ParseIntError;

fn leer(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

fn main() {
    let bien = leer("21");
    let mal = leer("veinte");

    println!("is_ok_and: {}  is_err_and: {}", bien.clone().is_ok_and(|x| x > 20), mal.clone().is_err_and(|e| e.to_string().contains("digit")));
    println!("map: {:?}  map_err: {:?}", bien.clone().map(|x| x * 2), mal.clone().map_err(|e| e.to_string()));
    println!("map_or: {}  unwrap_or_default: {}", mal.clone().map_or(-1, |x| x), mal.clone().unwrap_or_default());
    println!("ok: {:?}  err: {:?}", bien.clone().ok(), mal.clone().err().map(|e| e.to_string()));

    // inspect / inspect_err: mirar sin cambiar nada (útil para registrar).
    let _ = mal.clone().inspect_err(|e| println!("inspect_err: falló con «{e}»"));

    // and_then encadena pasos que pueden fallar; or_else intenta otra vía.
    let doble_positivo = bien.clone().and_then(|x| if x > 0 { Ok(x * 2) } else { leer("x") });
    println!("and_then: {doble_positivo:?}");
    let con_respaldo = mal.clone().or_else(|_| leer("7"));
    println!("or_else: {con_respaldo:?}");

    // Colecciones de Result: collect se detiene en el primer error.
    let todos: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| leer(s)).collect();
    let alguno_mal: Result<Vec<i32>, _> = ["1", "x", "3"].iter().map(|s| leer(s)).collect();
    println!("collect: {todos:?}  {}", alguno_mal.is_err());

    // Sumar los que están bien e ignorar los errores: flatten sobre los Ok.
    let suma: i32 = ["4", "no", "6"].iter().map(|s| leer(s)).flatten().sum();
    println!("flatten de Ok: {suma}");

    // transpose: de Result<Option> a Option<Result>.
    let r: Result<Option<i32>, ()> = Ok(Some(3));
    println!("transpose: {:?}", r.transpose());
}
