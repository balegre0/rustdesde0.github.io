fn main() {
    let mut nombres = vec![String::from("ana"), String::from("luis"), String::from("zoe")];

    // 1. iter() / for x in &v: préstamo compartido de cada elemento (&String).
    for n in &nombres {
        print!("{} ", n.len());
    }
    println!();

    // 2. iter_mut() / for x in &mut v: préstamo exclusivo (&mut String): se pueden cambiar.
    for n in &mut nombres {
        let mayus = n.to_uppercase();
        *n = mayus;
    }
    println!("{:?}", nombres);

    // 3. into_iter() / for x in v: CONSUME el Vec; cada x es un String propio.
    let mut todos = String::new();
    for n in nombres {
        todos.push_str(&n); // n es nuestro: podríamos guardarlo donde quisiéramos
    }
    println!("{todos}");
    // Aquí `nombres` ya no existe: se movió dentro del for.

    // 4. Lo mismo con HashMap: iterar &mapa da (&K, &V).
    let mut precios = std::collections::BTreeMap::new();
    precios.insert("pan", 1.20);
    precios.insert("leche", 0.95);
    for (producto, precio) in &mut precios {
        *precio *= 1.10;
        println!("{producto}: {precio:.2}");
    }

    // 5. Con índice: enumerate.
    for (i, c) in "ñu".chars().enumerate() {
        print!("{i}={c} ");
    }
    println!();
}
