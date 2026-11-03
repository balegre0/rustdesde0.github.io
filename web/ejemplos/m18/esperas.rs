// Simulación: tres pedidos que pasan casi todo su tiempo ESPERANDO
// (a la red, al disco...). El trabajo de CPU de cada uno es 1 tic.
struct Pedido {
    nombre: &'static str,
    espera: u32, // tics esperando una respuesta
}

fn main() {
    let pedidos = [
        Pedido { nombre: "A", espera: 5 },
        Pedido { nombre: "B", espera: 3 },
        Pedido { nombre: "C", espera: 4 },
    ];

    // 1. Uno detrás de otro: mientras A espera, nadie hace nada
    let mut reloj = 0;
    for p in &pedidos {
        reloj += 1 + p.espera; // 1 tic de trabajo + su espera
        println!("secuencial: {} termina en el tic {reloj}", p.nombre);
    }
    println!("secuencial total: {reloj} tics\n");

    // 2. Intercalando: lanzo los tres (1 tic cada uno) y espero A LA VEZ
    let mut fin_max = 0;
    for (i, p) in pedidos.iter().enumerate() {
        let empieza = i as u32; // el tic en que le toca su trabajo
        let fin = empieza + 1 + p.espera;
        println!("intercalado: {} termina en el tic {fin}", p.nombre);
        fin_max = fin_max.max(fin);
    }
    println!("intercalado total: {fin_max} tics");
}
