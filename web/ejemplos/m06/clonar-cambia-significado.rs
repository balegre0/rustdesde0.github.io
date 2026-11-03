fn main() {
    let mut compra = vec!["pan", "leche"];
    // 1. Queríamos añadir a `compra`... pero «para que compile» clonamos
    let mut lista = compra.clone();
    lista.push("huevos");

    // 2. Compila, pero `compra` no tiene los huevos
    println!("compra = {compra:?}");
    println!("lista  = {lista:?}");

    // 3. Lo que de verdad queríamos: modificar el original
    compra.push("huevos");
    println!("compra = {compra:?}");
}
