fn main() {
    let c = Cache::new();
    let cuadrado = |x: u64| x * x;
    let mut fallos = 0;
    let pedidos = [(3, 9, 1), (4, 16, 2), (3, 9, 2), (3, 9, 2), (10, 100, 3)];
    for (n, valor, calculos) in pedidos {
        let v = c.obtener(n, cuadrado);
        if v == valor && c.calculos() == calculos {
            println!("OK: obtener({n}) = {v}, cálculos: {}", c.calculos());
        } else {
            println!("FALLO: obtener({n}): esperaba {valor} y {calculos} cálculos, obtuve {v} y {}", c.calculos());
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
