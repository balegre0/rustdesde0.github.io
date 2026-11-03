fn main() {
    let mut invitados: Vec<String> = Vec::new();
    let ana = String::from("Ana");
    // 1. `push` se queda con el String: la propiedad pasa al Vec
    invitados.push(ana);
    invitados.push(String::from("Luis"));

    // 2. Mirar sin quitar: `&` crea una vista prestada (mundo 07)
    let primero = &invitados[0];
    println!("primero: {primero}");

    // 3. Sacar de verdad un elemento: `remove` lo entrega con su propiedad
    let sacado = invitados.remove(0);
    println!("sacado: {sacado}; quedan: {invitados:?}");
} // 4. Se libera `sacado`; luego el Vec, que libera a "Luis"
