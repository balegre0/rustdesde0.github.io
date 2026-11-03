fn main() {
    // 1. `a` nace aquí y vive hasta la llave que cierra `main`
    let a = String::from("pan");
    {
        // 2. `b` nace dentro del bloque interior
        let b = String::from("queso");
        println!("dentro: {a} y {b}");
    } // 3. aquí termina el ámbito de `b`: su texto en el heap se libera
    println!("fuera: {a}");
} // 4. aquí termina el ámbito de `a`: se libera su texto
