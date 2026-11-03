fn main() {
    let mut puntos = vec![10, 20, 30];

    // 1. Préstamo compartido: `primero` apunta dentro del Vec
    let primero = &puntos[0];
    println!("primero = {primero}");
    // 2. Último uso de `primero` arriba: para NLL el préstamo YA terminó aquí,
    //    aunque la variable siga «en ámbito» hasta el final de main.

    // 3. Así que este préstamo exclusivo es legal
    puntos.push(40);
    println!("puntos = {puntos:?}");

    // 4. Two-phase borrow: el &mut de push se «reserva», se evalúa len() y luego se activa
    puntos.push(puntos.len());
    println!("puntos = {puntos:?}");
}
