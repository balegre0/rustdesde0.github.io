fn main() {
    // Puntuaciones de 6 rondas de un juego (pueden ser negativas)
    let rondas = [12, -3, 0, 25, 7, -10];
    let mut total = 0;

    for i in 0..rondas.len() {
        let puntos = rondas[i];
        // 1. `match` con rangos y guarda para describir cada ronda
        let nota = match puntos {
            0 => "empate",
            p if p < 0 => "penalización",
            1..=9 => "bien",
            _ => "¡excelente!",
        };
        total += puntos;
        println!("ronda {}: {puntos:>3} -> {nota}", i + 1);
    }

    // 2. `if` como expresión para el veredicto
    let veredicto = if total >= 30 { "ganas" } else { "pierdes" };
    println!("total = {total}: {veredicto}");

    // 3. `loop` con `break` valor: ¿cuántas rondas de 7 harían falta para llegar a 100?
    let mut acumulado = total;
    let extra = loop {
        if acumulado >= 100 {
            break (acumulado - total) / 7;
        }
        acumulado += 7;
    };
    println!("rondas extra de 7 puntos para llegar a 100: {extra}");
}
