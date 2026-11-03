// 1. El estado del iterador: dónde vamos y dónde paramos.
struct CuentaAtras {
    actual: u32,
}

// 2. Implementar Iterator = decir qué tipo entrega (Item) y escribir next().
impl Iterator for CuentaAtras {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.actual == 0 {
            None
        } else {
            let valor = self.actual;
            self.actual -= 1;
            Some(valor)
        }
    }
}

fn main() {
    // 3. Ya funciona en un for...
    for n in (CuentaAtras { actual: 3 }) {
        print!("{n}... ");
    }
    println!("¡despegue!");

    // 4. ...y regala TODOS los adaptadores: solo escribimos next().
    let suma_pares: u32 = CuentaAtras { actual: 10 }.filter(|n| n % 2 == 0).sum();
    println!("suma de pares de 10 a 1 = {suma_pares}");
    let v: Vec<u32> = CuentaAtras { actual: 5 }.map(|n| n * 100).collect();
    println!("{v:?}");
}
