fn main() {
    // 1. Crear: vacío (con el tipo dicho) o con la macro vec!.
    let mut notas: Vec<u32> = Vec::new();
    let dias = vec!["lun", "mar", "mié"];
    let ceros = vec![0; 5]; // cinco ceros

    // 2. Añadir al final y quitar del final.
    notas.push(7);
    notas.push(9);
    notas.push(4);
    println!("notas: {:?} (len {})", notas, notas.len());
    println!("pop: {:?}", notas.pop());

    // 3. Leer: con [i] (pánico si no existe) o con get(i) (Option).
    println!("dias[1] = {}", dias[1]);
    println!("dias.get(7) = {:?}", dias.get(7));
    println!("primero {:?}, último {:?}", dias.first(), dias.last());

    // 4. Cambiar un elemento.
    notas[0] = 10;

    // 5. Insertar y quitar en medio (desplaza los demás).
    notas.insert(1, 8);
    let quitado = notas.remove(0);
    println!("quitado {quitado}; ahora {:?}", notas);

    // 6. Preguntas útiles.
    println!("¿vacío? {}  ¿contiene 9? {}", ceros.is_empty(), notas.contains(&9));

    // 7. Ordenar y recorrer.
    let mut desorden = vec![3, 1, 2];
    desorden.sort();
    for n in &desorden {
        print!("{n} ");
    }
    println!();
    println!("ceros: {:?}", ceros);
}
