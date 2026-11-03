
fn main() {
    use Token::*;
    let mut fallos = 0;
    let mut caso = |nombre: &str, obtenido: Result<Vec<Token>, String>, esperado: Result<Vec<Token>, String>| {
        if obtenido == esperado { println!("OK: {nombre}"); } else { fallos += 1; println!("FALLO: {nombre}: esperaba {esperado:?}, obtuve {obtenido:?}"); }
    };
    caso("let simple", tokenizar("let x1 = 42;"), Ok(vec![Clave("let".into()), Ident("x1".into()), Simbolo('='), Num(42), Simbolo(';')]));
    caso("vacío", tokenizar("   \n\t "), Ok(vec![]));
    caso("expresión", tokenizar("(a+b)*10"), Ok(vec![Simbolo('('), Ident("a".into()), Simbolo('+'), Ident("b".into()), Simbolo(')'), Simbolo('*'), Num(10)]));
    caso("fn y guion bajo", tokenizar("fn _sumar"), Ok(vec![Clave("fn".into()), Ident("_sumar".into())]));
    caso("letra no es clave", tokenizar("lets"), Ok(vec![Ident("lets".into())]));
    caso("error con posición", tokenizar("x = 3 $ 4"), Err("carácter inesperado '$' en 6".into()));
    caso("posición en bytes", tokenizar("ñ1 # 2"), Err("carácter inesperado '#' en 4".into()));
    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
