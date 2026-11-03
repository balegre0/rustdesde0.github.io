fn main() {
    let numeros = vec![1, 2, 3];

    // 1. Preparamos la receta. ¿Se imprime algo aquí?
    let receta = numeros.iter().map(|n| {
        println!("  calculando {n} * 10");
        n * 10
    });
    println!("receta preparada");

    // 2. Solo al consumirla se ejecuta, un elemento cada vez.
    for x in receta {
        println!("recibo {x}");
    }
}
