fn main() {
    // 1. Un bloque { } se evalúa: su valor es la última expresión, la que no lleva `;`
    let area = {
        let ancho = 4;
        let alto = 3;
        ancho * alto
    };
    println!("área = {area}");

    // 2. Las variables de dentro (`ancho`, `alto`) ya no existen aquí fuera
    // 3. Un bloque que termina en `;` vale `()`, la «tupla vacía»
    let nada = {
        let x = 1;
        println!("dentro del bloque, x = {x}");
    };
    println!("nada = {nada:?}");
}
