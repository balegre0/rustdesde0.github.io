// Jefe del mundo 14: un informe de ventas hecho solo con closures e iteradores.

struct Venta {
    producto: &'static str,
    unidades: u32,
    precio: u32, // céntimos por unidad
}

// 1. Un filtro configurable: devuelve un closure que recuerda el mínimo.
fn al_menos(minimo: u32) -> impl Fn(&&Venta) -> bool {
    move |v| v.unidades * v.precio >= minimo
}

// 2. Un iterador propio: numera líneas empezando en 1.
struct Numerador {
    siguiente: u32,
}

impl Iterator for Numerador {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let n = self.siguiente;
        self.siguiente += 1;
        Some(n)
    }
}

fn main() {
    let ventas = vec![
        Venta { producto: "café", unidades: 30, precio: 120 },
        Venta { producto: "té", unidades: 4, precio: 90 },
        Venta { producto: "churros", unidades: 12, precio: 250 },
        Venta { producto: "agua", unidades: 50, precio: 60 },
    ];

    // 3. Total con map + sum.
    let total: u32 = ventas.iter().map(|v| v.unidades * v.precio).sum();
    println!("Total: {}.{:02} €", total / 100, total % 100);

    // 4. Filtro configurable + zip con el iterador infinito.
    let filtro = al_menos(2000);
    for (n, v) in (Numerador { siguiente: 1 }).zip(ventas.iter().filter(filtro)) {
        println!("{n}. {} ({} uds.)", v.producto, v.unidades);
    }

    // 5. max_by_key y un FnMut que cuenta llamadas.
    let mut consultas = 0;
    let estrella = ventas
        .iter()
        .max_by_key(|v| {
            consultas += 1;
            v.unidades * v.precio
        })
        .map(|v| v.producto);
    println!("Estrella: {estrella:?} ({consultas} consultas)");
}
