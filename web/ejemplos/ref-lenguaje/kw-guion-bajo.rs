fn main() {
    // 1. En un patrón: «aquí hay algo, pero no me interesa».
    let (a, _) = (1, 2);
    println!("a = {a}");
    // 2. En match: el caso «cualquier otro valor».
    let n = 7;
    let tipo = match n {
        0 => "cero",
        _ => "otro",
    };
    println!("{tipo}");
    // 3. Como tipo: «dedúcelo tú, compilador».
    let v: Vec<_> = (1..=3).collect();
    println!("{v:?}");
    // 4. Descartar un valor a propósito.
    _ = "texto que no se usa".len();
    // 5. Separador en literales numéricos (aquí `_` no es palabra clave).
    let millon = 1_000_000;
    println!("{millon}");
}
