use std::mem::size_of_val;

fn main() {
    let n: u64 = 7;
    let texto = String::from("hola");

    // 1. Sin capturas: no guarda nada.
    let sin_nada = || 1 + 1;
    // 2. Captura `n` por referencia: guarda un puntero.
    let por_ref = || n + 1;
    // 3. Captura `n` por valor: guarda una copia del u64.
    let por_valor = move || n + 1;
    // 4. Se lleva la String entera: guarda (puntero, capacidad, longitud).
    let con_string = move || texto.len();

    println!("sin capturas:      {} bytes", size_of_val(&sin_nada));
    println!("&u64 capturado:    {} bytes", size_of_val(&por_ref));
    println!("u64 movido:        {} bytes", size_of_val(&por_valor));
    println!("String movida:     {} bytes", size_of_val(&con_string));
    println!("(un usize aquí mide {} bytes)", size_of::<usize>());
    let _ = (sin_nada(), por_ref(), por_valor(), con_string());
}
