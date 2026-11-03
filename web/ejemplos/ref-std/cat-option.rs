fn main() {
    let mut a: Option<i32> = Some(4);
    let nada: Option<i32> = None;

    // Preguntar con condición.
    println!("is_some_and: {}  is_none_or: {}", a.is_some_and(|x| x > 3), nada.is_none_or(|x| x > 3));

    // Transformar con valor por defecto.
    println!("map_or: {}  map_or_else: {}", a.map_or(0, |x| x * 10), nada.map_or_else(|| -1, |x| x * 10));

    // Combinar.
    println!("zip: {:?}  xor: {:?}  or: {:?}", a.zip(Some('x')), a.xor(nada), nada.or(Some(9)));
    println!("filter: {:?}  and_then: {:?}", a.filter(|x| x % 2 == 1), a.and_then(|x| x.checked_sub(10)));

    // De Option a Result.
    println!("ok_or: {:?}", nada.ok_or("faltaba el valor"));

    // Modificar en el sitio.
    let viejo = a.replace(7);
    println!("replace: devolvió {viejo:?}, ahora {a:?}");
    let sacado = a.take_if(|x| *x > 5);
    println!("take_if: sacó {sacado:?}, queda {a:?}");
    let r = a.get_or_insert_with(|| 100);
    *r += 1;
    println!("get_or_insert_with: {a:?}");

    // Mirar sin consumir, y préstamos.
    let nombre: Option<String> = Some(String::from("Lía"));
    let longitud = nombre.as_deref().map(str::len);
    println!("as_deref: {longitud:?} (nombre sigue: {nombre:?})");
    let _ = nombre.as_ref().inspect(|n| println!("inspect: vi {n}"));

    // Aplanar y transponer.
    println!("flatten: {:?}", Some(Some(3)).flatten());
    let t: Result<Option<i32>, String> = Some("42".parse::<i32>().map_err(|e| e.to_string())).transpose();
    println!("transpose: {t:?}");
    println!("as_slice: {:?}", Some(5).as_slice());
}
