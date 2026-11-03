fn main() {
    let mut fallos = 0;
    let casos: [(u64, &[u64]); 3] = [(6, &[6, 3, 10, 5, 16, 8, 4, 2, 1]), (1, &[1]), (4, &[4, 2, 1])];
    for (n, esperado) in casos {
        let r: Vec<u64> = collatz(n).collect();
        if r == esperado {
            println!("OK: collatz({n}) = {r:?}");
        } else {
            println!("FALLO: collatz({n}) = {r:?}, se esperaba {esperado:?}");
            fallos += 1;
        }
    }
    let largo = collatz(27).count();
    if largo == 112 {
        println!("OK: collatz(27).count() = 112");
    } else {
        println!("FALLO: collatz(27).count() = {largo}, se esperaba 112");
        fallos += 1;
    }
    let maximo = collatz(27).max();
    if maximo == Some(9232) {
        println!("OK: collatz(27).max() = Some(9232)");
    } else {
        println!("FALLO: collatz(27).max() = {maximo:?}, se esperaba Some(9232)");
        fallos += 1;
    }
    let mut it = collatz(2);
    let tras_fin = (it.next(), it.next(), it.next(), it.next());
    if tras_fin == (Some(2), Some(1), None, None) {
        println!("OK: tras el 1 siempre None");
    } else {
        println!("FALLO: collatz(2) dio {tras_fin:?}, se esperaba (Some(2), Some(1), None, None)");
        fallos += 1;
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
