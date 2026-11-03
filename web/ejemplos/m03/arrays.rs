use std::mem::size_of;

fn main() {
    // 1. Un array: N valores del MISMO tipo, seguidos en memoria
    let dias: [&str; 3] = ["lun", "mar", "mié"];
    let notas = [7, 9, 5, 10];
    // 2. [valor; N] repite un valor N veces
    let ceros = [0u8; 5];
    // 3. Índices desde 0; .len() da la longitud
    println!("primer día: {}", dias[0]);
    println!("última nota: {}", notas[notas.len() - 1]);
    println!("{:?} tiene {} elementos", ceros, ceros.len());
    // 4. Con mut se pueden cambiar elementos (no la longitud)
    let mut puntos = [0; 3];
    puntos[1] = 50;
    println!("{:?}", puntos);
    // 5. El tamaño es parte del tipo: [i32; 4] ocupa 4 × 4 bytes
    println!("[i32; 4] ocupa {} bytes", size_of::<[i32; 4]>());
}
