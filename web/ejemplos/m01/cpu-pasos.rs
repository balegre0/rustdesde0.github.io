// La CPU ejecuta las instrucciones de una en una, en orden.
fn main() {
    let mut acumulador = 0;
    acumulador = acumulador + 5; // 1. suma 5
    acumulador = acumulador * 2; // 2. multiplica por 2
    acumulador = acumulador - 3; // 3. resta 3
    println!("{acumulador}");
}
