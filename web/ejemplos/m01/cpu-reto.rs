// Cambia SOLO la memoria para que la CPU termine con A=10.
// 1 = cargar el dato siguiente en A, 2 = A + B en A, 3 = copiar A en B, 0 = parar.
fn main() {
    // cargar 5, copiarlo en B, sumar A + B (5 + 5 = 10), parar
    let memoria = [1, 5, 3, 2, 0];
    let mut a = 0;
    let mut b = 0;
    let mut pc = 0;

    loop {
        let instruccion = memoria[pc];
        pc = pc + 1;
        if instruccion == 1 {
            a = memoria[pc];
            pc = pc + 1;
        } else if instruccion == 2 {
            a = a + b;
        } else if instruccion == 3 {
            b = a;
        } else {
            break;
        }
    }
    println!("A={a}");
}
