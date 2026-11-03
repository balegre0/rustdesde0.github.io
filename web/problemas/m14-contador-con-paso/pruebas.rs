fn main() {
    let mut fallos = 0;
    let mut comprobar = |nombre: &str, r: Vec<i32>, esperado: &[i32]| {
        if r == esperado {
            println!("OK: {nombre} = {r:?}");
        } else {
            println!("FALLO: {nombre} = {r:?}, se esperaba {esperado:?}");
            fallos += 1;
        }
    };
    let mut c = hacer_contador(10, 5);
    comprobar("hacer_contador(10, 5), tres llamadas", vec![c(), c(), c()], &[10, 15, 20]);
    let mut a = hacer_contador(0, 1);
    let mut b = hacer_contador(0, 1);
    let primeras = vec![a(), a(), a(), b()];
    comprobar("dos contadores independientes", primeras, &[0, 1, 2, 0]);
    let mut abajo = hacer_contador(3, -2);
    comprobar("hacer_contador(3, -2)", vec![abajo(), abajo(), abajo()], &[3, 1, -1]);
    let mut quieto = hacer_contador(7, 0);
    comprobar("hacer_contador(7, 0)", vec![quieto(), quieto()], &[7, 7]);
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
