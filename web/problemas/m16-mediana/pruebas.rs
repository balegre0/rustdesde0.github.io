
struct Azar(u64);

impl Azar {
    fn siguiente(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn main() {
    let casos: Vec<(&str, Vec<f64>, Option<f64>)> = vec![
        ("vacía", vec![], None),
        ("un elemento", vec![7.0], Some(7.0)),
        ("impar desordenada", vec![3.0, 1.0, 2.0], Some(2.0)),
        ("par desordenada", vec![4.0, 1.0, 3.0, 2.0], Some(2.5)),
        ("repetidos", vec![5.0, 5.0, 1.0, 5.0], Some(5.0)),
        ("negativos", vec![-1.0, -10.0, 4.0], Some(-1.0)),
        ("dos elementos", vec![10.0, 20.0], Some(15.0)),
    ];
    let mut fallos = 0;
    for (nombre, v, esperado) in &casos {
        let original = v.clone();
        let obtenido = mediana(v);
        if obtenido == *esperado && *v == original {
            println!("OK: {nombre}: mediana({v:?}) = {obtenido:?}");
        } else {
            println!("FALLO: {nombre}: mediana({original:?}) debía ser {esperado:?}, salió {obtenido:?}");
            fallos += 1;
        }
    }
    // Propiedad: barajar la lista no cambia la mediana.
    let mut azar = Azar(5);
    let mut propiedad_ok = true;
    for _ in 0..300 {
        let n = 1 + (azar.siguiente() % 9) as usize;
        let v: Vec<f64> = (0..n).map(|_| (azar.siguiente() % 50) as f64 - 25.0).collect();
        let mut barajada = v.clone();
        for i in (1..barajada.len()).rev() {
            let j = (azar.siguiente() % (i as u64 + 1)) as usize;
            barajada.swap(i, j);
        }
        if mediana(&v) != mediana(&barajada) {
            println!("FALLO: propiedad: {v:?} y {barajada:?} dan medianas distintas");
            propiedad_ok = false;
            break;
        }
    }
    if propiedad_ok {
        println!("OK: propiedad con 300 listas barajadas");
    } else {
        fallos += 1;
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
