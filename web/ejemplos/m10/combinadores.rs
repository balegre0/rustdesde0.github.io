use std::num::ParseIntError;

fn main() {
    // 1. map: transformar el valor de Ok (el Err pasa intacto).
    let r: Result<i32, ParseIntError> = "21".parse::<i32>().map(|n| n * 2);
    println!("map: {:?}", r);

    // 2. map_err: transformar el ERROR (el Ok pasa intacto).
    let r2: Result<i32, String> = "x".parse::<i32>().map_err(|e| format!("entrada mala: {e}"));
    println!("map_err: {:?}", r2);

    // 3. and_then: encadenar otra operación que también puede fallar.
    let r3 = "16".parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|n| if n >= 0 { Ok(n.isqrt()) } else { Err(String::from("negativo")) });
    println!("and_then: {:?}", r3);

    // 4. ok_or: de Option a Result, poniendo el error que falta.
    let nombres = ["Ana", "Íker"];
    let r4: Result<&&str, &str> = nombres.get(5).ok_or("no hay quinto nombre");
    println!("ok_or: {:?}", r4);

    // 5. ok: de Result a Option (se pierde el error).
    println!("ok: {:?}", "7".parse::<u8>().ok());

    // 6. collect sobre Result: o todos Ok, o el PRIMER error.
    let bien: Result<Vec<u8>, _> = ["1", "2", "3"].iter().map(|t| t.parse::<u8>()).collect();
    let mal: Result<Vec<u8>, _> = ["1", "dos", "300"].iter().map(|t| t.parse::<u8>()).collect();
    println!("collect: {:?} / {:?}", bien, mal);

    // 7. unwrap_or_else con el error a mano.
    let n = "?".parse::<i32>().unwrap_or_else(|e| {
        println!("  recupero de: {e}");
        0
    });
    println!("unwrap_or_else: {n}");
}
