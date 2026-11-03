use std::collections::HashMap;

fn main() {
    // 1. Crear e insertar: clave → valor.
    let mut capitales: HashMap<String, String> = HashMap::new();
    capitales.insert(String::from("España"), String::from("Madrid"));
    capitales.insert(String::from("Perú"), String::from("Lima"));
    capitales.insert(String::from("Chile"), String::from("Santiago"));

    // 2. Buscar: get devuelve Option<&V>. Se puede buscar con &str aunque la clave sea String.
    match capitales.get("Perú") {
        Some(c) => println!("capital de Perú: {c}"),
        None => println!("no la sé"),
    }
    println!("¿está Francia? {}", capitales.contains_key("Francia"));

    // 3. insert sobre una clave existente REEMPLAZA y devuelve el valor viejo.
    let viejo = capitales.insert(String::from("Chile"), String::from("Santiago de Chile"));
    println!("valor anterior: {:?}", viejo);

    // 4. remove devuelve el valor quitado.
    println!("quitado: {:?}; quedan {}", capitales.remove("España"), capitales.len());

    // 5. La API entry: «si no está, inserta; en cualquier caso, dame acceso».
    let texto = "el sol y el mar y el cielo";
    let mut cuenta: HashMap<&str, u32> = HashMap::new();
    for palabra in texto.split_whitespace() {
        *cuenta.entry(palabra).or_insert(0) += 1;
    }

    // 6. El orden de recorrido de un HashMap NO está definido: ordenamos para imprimir.
    let mut pares: Vec<(&str, u32)> = cuenta.into_iter().collect();
    pares.sort();
    println!("{:?}", pares);

    // 7. Agrupar con entry + or_default: cada clave, un Vec.
    let mut por_inicial: HashMap<char, Vec<&str>> = HashMap::new();
    for nombre in ["Ana", "Íker", "Alba", "Iris", "Ángel"] {
        let inicial = nombre.chars().next().unwrap();
        por_inicial.entry(inicial).or_default().push(nombre);
    }
    let mut claves: Vec<_> = por_inicial.keys().collect();
    claves.sort();
    for k in claves {
        println!("{k}: {:?}", por_inicial[k]);
    }
}
