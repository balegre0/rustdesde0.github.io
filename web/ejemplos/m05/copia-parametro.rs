// `mut` en el parámetro: la función puede cambiar SU copia
fn subir(mut nivel: u32) {
    nivel += 10;
    println!("  dentro de subir: nivel = {nivel}");
}

fn main() {
    let nivel = 1;
    // 1. Se pasa una copia del valor 1
    subir(nivel);
    // 2. La variable de main no ha cambiado
    println!("en main: nivel = {nivel}");
}
