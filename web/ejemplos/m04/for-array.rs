fn main() {
    let notas = [7, 9, 4, 10];
    let mut suma = 0;

    // 1. `for` recorre cada elemento; no hay índice que se salga del array
    for nota in notas {
        suma += nota;
    }
    println!("suma = {suma}, media = {}", suma as f64 / notas.len() as f64);

    // 2. Si también quieres la posición: `.iter().enumerate()`
    for (i, nota) in notas.iter().enumerate() {
        println!("nota {i}: {nota}");
    }

    // 3. El texto se recorre carácter a carácter con `.chars()`
    for c in "año".chars() {
        print!("[{c}]");
    }
    println!();
}
