
fn main() {
    let mut fallos = 0;
    let mut comprueba = |nombre: &str, obtenido: Vec<String>, esperado: &[&str]| {
        if obtenido == esperado {
            println!("OK: {nombre} = {obtenido:?}");
        } else {
            println!("FALLO: {nombre} = {obtenido:?}, se esperaba {esperado:?}");
            fallos += 1;
        }
    };
    let mut m = Mapa { estaciones: Vec::new() };
    let sol = m.nueva_estacion("Sol");
    let opera = m.nueva_estacion("Ópera");
    let callao = m.nueva_estacion("Callao");
    let retiro = m.nueva_estacion("Retiro");
    let ok_indices = sol == 0 && opera == 1 && callao == 2 && retiro == 3;
    if ok_indices {
        println!("OK: índices 0, 1, 2, 3");
    } else {
        println!("FALLO: los índices deberían ser 0, 1, 2, 3 y son {sol}, {opera}, {callao}, {retiro}");
    }
    m.unir(sol, opera);
    m.unir(opera, callao);
    comprueba("alcanzables(Sol)", m.alcanzables(sol), &["Callao", "Sol", "Ópera"]);
    comprueba("alcanzables(Callao)", m.alcanzables(callao), &["Callao", "Sol", "Ópera"]);
    comprueba("alcanzables(Retiro)", m.alcanzables(retiro), &["Retiro"]);
    m.unir(callao, sol); // un ciclo: Sol → Ópera → Callao → Sol
    comprueba("con ciclo, alcanzables(Ópera)", m.alcanzables(opera), &["Callao", "Sol", "Ópera"]);
    let atocha = m.nueva_estacion("Atocha");
    m.unir(retiro, atocha);
    comprueba("alcanzables(Atocha)", m.alcanzables(atocha), &["Atocha", "Retiro"]);
    m.unir(atocha, sol);
    comprueba("todo unido", m.alcanzables(retiro), &["Atocha", "Callao", "Retiro", "Sol", "Ópera"]);
    if fallos == 0 && ok_indices {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
