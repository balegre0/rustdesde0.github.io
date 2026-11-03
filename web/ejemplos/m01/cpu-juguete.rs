// Una CPU de juguete: dos registros (A y B) y un programa de 4 instrucciones.
// Cada instrucción es un número: 1 = cargar en A, 2 = sumar A + B y guardar en A,
// 3 = copiar A en B, 0 = parar. El número que sigue a la instrucción 1 es el dato.
fn main() {
    let memoria = [1, 7, 3, 1, 5, 2, 0];
    let mut a = 0; // registro A
    let mut b = 0; // registro B
    let mut pc = 0; // contador de programa: dónde está la próxima instrucción

    loop {
        // 1. Buscar (fetch): leer la instrucción a la que apunta pc
        let instruccion = memoria[pc];
        pc = pc + 1;
        // 2. Decodificar (decode) y 3. ejecutar (execute)
        if instruccion == 1 {
            a = memoria[pc];
            pc = pc + 1;
            println!("cargar {a} en A        -> A={a} B={b}");
        } else if instruccion == 2 {
            a = a + b;
            println!("sumar A + B en A      -> A={a} B={b}");
        } else if instruccion == 3 {
            b = a;
            println!("copiar A en B         -> A={a} B={b}");
        } else {
            println!("parar");
            break;
        }
    }
}
