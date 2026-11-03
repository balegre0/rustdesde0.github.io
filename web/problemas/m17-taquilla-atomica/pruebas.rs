fn main() {
    let t = Taquilla::new(5);
    let pedidos = [(2, true, 3), (2, true, 1), (2, false, 1), (1, true, 0), (1, false, 0), (0, true, 0)];
    let mut fallos = 0;
    for (n, ok, quedan) in pedidos {
        let vendido = t.vender(n);
        if vendido == ok && t.quedan() == quedan {
            println!("OK: vender({n}) -> {vendido}, quedan {}", t.quedan());
        } else {
            println!("FALLO: vender({n}): esperaba {ok} y {quedan}, obtuve {vendido} y {}", t.quedan());
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
