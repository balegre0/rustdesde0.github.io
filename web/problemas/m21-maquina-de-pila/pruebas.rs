
fn main() {
    use Instr::*;
    let mut fallos = 0;
    let mut caso = |nombre: &str, obtenido: Result<i64, String>, esperado: Result<i64, String>| {
        if obtenido == esperado { println!("OK: {nombre}"); } else { fallos += 1; println!("FALLO: {nombre}: esperaba {esperado:?}, obtuve {obtenido:?}"); }
    };
    caso("resta en orden", ejecutar(&[Empuja(7), Empuja(2), Resta]), Ok(5));
    caso("x*5-4 con x=10", ejecutar(&[Empuja(10), Empuja(5), Mul, Empuja(4), Resta]), Ok(46));
    caso("división entera", ejecutar(&[Empuja(-9), Empuja(2), Div]), Ok(-4));
    caso("pila vacía", ejecutar(&[Empuja(1), Suma]), Err("pila vacía".into()));
    caso("división por cero", ejecutar(&[Empuja(1), Empuja(0), Div]), Err("división por cero".into()));
    caso("sobran valores", ejecutar(&[Empuja(1), Empuja(2)]), Err("la pila termina con 2 valores".into()));
    caso("programa vacío", ejecutar(&[]), Err("la pila termina con 0 valores".into()));
    caso("desbordamiento", ejecutar(&[Empuja(i64::MAX), Empuja(1), Suma]), Err("desbordamiento".into()));
    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
