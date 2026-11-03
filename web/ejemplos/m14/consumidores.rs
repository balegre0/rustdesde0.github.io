use std::collections::HashMap;

fn main() {
    let precios = vec![12, 5, 30, 7];

    // 1. Resumir en un número.
    let total: i32 = precios.iter().sum();
    let cuantos = precios.iter().count();
    let maximo = precios.iter().max();
    println!("sum = {total}, count = {cuantos}, max = {maximo:?}");

    // 2. fold: tú decides cómo se acumula (empieza en 1 y multiplica).
    let producto = precios.iter().fold(1, |acc, x| acc * x);
    println!("fold (producto) = {producto}");

    // 3. Preguntar: any, all, find, position.
    println!("¿alguno > 20? {}", precios.iter().any(|&p| p > 20));
    println!("¿todos > 1? {}", precios.iter().all(|&p| p > 1));
    println!("primero < 10: {:?}", precios.iter().find(|&&p| p < 10));
    println!("posición del 30: {:?}", precios.iter().position(|&p| p == 30));

    // 4. collect a distintas colecciones: el TIPO decide qué se construye.
    let texto: String = ['h', 'o', 'l', 'a'].iter().collect();
    let tabla: HashMap<&str, i32> = vec![("pan", 2), ("leche", 1)].into_iter().collect();
    println!("{texto} {:?}", tabla.get("pan"));

    // 5. Turbofish: el tipo dentro de la llamada.
    let impares = (1..8).filter(|n| n % 2 == 1).collect::<Vec<_>>();
    println!("{impares:?}");
}
