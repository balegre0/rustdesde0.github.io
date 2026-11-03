
fn main() {
    use Op::*;
    use Sentencia::*;
    use Terminador::*;
    let mut fallos = 0;
    let mut caso = |nombre: &str, obtenido: Result<i64, String>, esperado: Result<i64, String>| {
        if obtenido == esperado { println!("OK: {nombre}"); } else { fallos += 1; println!("FALLO: {nombre}: esperaba {esperado:?}, obtuve {obtenido:?}"); }
    };
    // factorial(_1): bb0: _0 = 1 -> bb1 | bb1: _1 == 0 ? bb3 : bb2 | bb2: _0 *= _1; _1 -= 1 -> bb1 | bb3: return
    let fact = vec![
        Bloque { sentencias: vec![Asigna(0, Const(1))], terminador: Goto(1) },
        Bloque { sentencias: vec![], terminador: SwitchInt(1, 0, 3, 2) },
        Bloque { sentencias: vec![Mul(0, Local(0), Local(1)), Suma(1, Local(1), Const(-1))], terminador: Goto(1) },
        Bloque { sentencias: vec![], terminador: Return },
    ];
    caso("factorial(5)", ejecutar(&fact, &[5]), Ok(120));
    caso("factorial(0)", ejecutar(&fact, &[0]), Ok(1));
    // _0 = _1 * 2 y assert sobre la bandera _2
    let con_assert = vec![
        Bloque { sentencias: vec![Mul(0, Local(1), Const(2))], terminador: Assert(2, "attempt to multiply with overflow".into()) },
        Bloque { sentencias: vec![], terminador: Return },
    ];
    caso("assert pasa", ejecutar(&con_assert, &[21, 1]), Ok(42));
    caso("assert falla", ejecutar(&con_assert, &[21, 0]), Err("attempt to multiply with overflow".into()));
    let suma = vec![Bloque { sentencias: vec![Suma(0, Local(1), Local(2))], terminador: Return }];
    caso("suma", ejecutar(&suma, &[40, 2]), Ok(42));
    let infinito = vec![Bloque { sentencias: vec![], terminador: Goto(0) }];
    caso("bucle infinito", ejecutar(&infinito, &[]), Err("demasiados pasos".into()));
    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
