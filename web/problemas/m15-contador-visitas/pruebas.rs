
fn prueba_visita_compartida(p: &Pagina, veces: u32) -> u32 {
    let mut ultimo = 0;
    for _ in 0..veces {
        ultimo = p.visitar();
    }
    ultimo
}

fn main() {
    let mut fallos = 0;
    let mut comprueba = |nombre: &str, obtenido: u32, esperado: u32| {
        if obtenido == esperado {
            println!("OK: {nombre} = {obtenido}");
        } else {
            println!("FALLO: {nombre} = {obtenido}, se esperaba {esperado}");
            fallos += 1;
        }
    };
    let p = Pagina::nueva("inicio");
    let a = &p;
    let b = &p; // dos préstamos compartidos a la vez
    comprueba("primera visita", a.visitar(), 1);
    comprueba("segunda visita (desde otro préstamo)", b.visitar(), 2);
    comprueba("reiniciar devuelve las que había", a.reiniciar(), 2);
    comprueba("visita tras reiniciar", b.visitar(), 1);
    comprueba("100 visitas más", prueba_visita_compartida(&p, 100), 101);
    comprueba("reiniciar otra vez", p.reiniciar(), 101);
    comprueba("reiniciar dos veces seguidas", p.reiniciar(), 0);
    let q = Pagina::nueva("otra");
    comprueba("las páginas no comparten contador", q.visitar(), 1);
    let _ = (&p.titulo, &q.titulo);
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
