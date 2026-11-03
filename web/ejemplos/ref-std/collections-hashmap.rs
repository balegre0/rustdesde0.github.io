use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    // 1. insertar (devuelve el valor anterior, si lo había)
    stock.insert(String::from("pan"), 3);
    stock.insert(String::from("leche"), 2);
    let antes = stock.insert(String::from("pan"), 5);
    println!("{antes:?}");
    // 2. leer
    println!("{:?} {:?}", stock.get("pan"), stock.get("café"));
    println!("{}", stock.contains_key("leche"));
    // 3. la API entry: insertar o actualizar en un paso
    let texto = "a b a c b a";
    let mut cuenta: HashMap<&str, u32> = HashMap::new();
    for p in texto.split(' ') {
        *cuenta.entry(p).or_insert(0) += 1;
    }
    // el orden de un HashMap no está definido: ordenamos para imprimir
    let mut pares: Vec<_> = cuenta.iter().collect();
    pares.sort();
    println!("{pares:?}");
    // 4. quitar
    println!("{:?} {}", stock.remove("leche"), stock.len());
}
