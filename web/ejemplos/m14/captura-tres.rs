fn main() {
    // 1. Captura por préstamo compartido: solo lee.
    let nombre = String::from("Lucía");
    let leer = || println!("leo: {nombre}");
    leer();
    println!("sigo pudiendo usar nombre: {nombre}");

    // 2. Captura por préstamo exclusivo: modifica.
    let mut visitas = 0;
    let mut visitar = || visitas += 1;
    visitar();
    visitar();
    println!("visitas = {visitas}");

    // 3. Captura por valor (mover) con `move`.
    let mensaje = String::from("me voy con el closure");
    let llevar = move || println!("{mensaje}");
    llevar();
    // `mensaje` ya no es de main: ahora es del closure.
}
