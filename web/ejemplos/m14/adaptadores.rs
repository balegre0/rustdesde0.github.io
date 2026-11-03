fn main() {
    let v = vec![3, 8, 1, 6, 4];

    // 1. map: transforma cada elemento.
    let dobles: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map:       {dobles:?}");

    // 2. filter: deja pasar solo los que cumplen la condición.
    let grandes: Vec<&i32> = v.iter().filter(|x| **x > 3).collect();
    println!("filter:    {grandes:?}");

    // 3. enumerate: añade la posición.
    for (i, x) in v.iter().enumerate().take(2) {
        println!("enumerate: v[{i}] = {x}");
    }

    // 4. zip: empareja dos iteradores (se para en el más corto).
    let nombres = ["Ana", "Leo", "Sara"];
    let parejas: Vec<(&str, &i32)> = nombres.into_iter().zip(v.iter()).collect();
    println!("zip:       {parejas:?}");

    // 5. take y skip: los primeros n / sáltate los primeros n.
    let medio: Vec<&i32> = v.iter().skip(1).take(3).collect();
    println!("skip+take: {medio:?}");

    // 6. chain: uno detrás de otro.
    let todo: Vec<i32> = (1..=2).chain(8..10).collect();
    println!("chain:     {todo:?}");

    // 7. flat_map: cada elemento produce varios y se aplanan.
    let letras: Vec<char> = ["ab", "ñ"].iter().flat_map(|s| s.chars()).collect();
    println!("flat_map:  {letras:?}");

    // 8. rev: al revés.
    let alreves: Vec<&i32> = v.iter().rev().collect();
    println!("rev:       {alreves:?}");
}
