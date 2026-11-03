fn main() {
    let buenos = [("2 + 3 * 4", 14), ("(2 + 3) * 4", 20), ("10 - 3 - 2", 5), ("100 / 10 / 5", 2), ("7", 7), (" ( ( 1 ) ) ", 1), ("2*(3+4)*5 - 1", 69), ("9 / 2", 4)];
    let mut fallos = 0;
    for (t, esperado) in buenos {
        match evaluar(t) {
            Ok(v) if v == esperado => println!("OK: {t:?} = {v}"),
            otro => {
                println!("FALLO: {t:?}: esperaba Ok({esperado}), obtuve {otro:?}");
                fallos += 1;
            }
        }
    }
    match evaluar("5 / (3 - 3)") {
        Err(e) if e == "división entre cero" => println!("OK: división entre cero detectada"),
        otro => {
            println!("FALLO: 5 / (3 - 3): esperaba Err(\"división entre cero\"), obtuve {otro:?}");
            fallos += 1;
        }
    }
    for malo in ["", "2 +", "(1 + 2", "1 + 2)", "3 $ 4", "4 5"] {
        match evaluar(malo) {
            Err(_) => println!("OK: {malo:?} rechazado"),
            Ok(v) => {
                println!("FALLO: {malo:?} debía dar Err, dio Ok({v})");
                fallos += 1;
            }
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
