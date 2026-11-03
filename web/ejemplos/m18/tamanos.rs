use std::mem::size_of_val;

async fn paso() {}

// 1. Guarda un array de 100 bytes que sigue vivo después del .await
async fn con_array(x: u64) -> u64 {
    let a = [7u8; 100];
    paso().await;
    x + a[0] as u64
}

// 2. La misma idea sin el array
async fn sin_array(x: u64) -> u64 {
    paso().await;
    x
}

fn main() {
    // 3. Crear el future no ejecuta nada: solo reserva su máquina de estados
    let f1 = con_array(1);
    let f2 = sin_array(1);
    println!("con array: {} bytes", size_of_val(&f1));
    println!("sin array: {} bytes", size_of_val(&f2));
    println!("paso():    {} bytes", size_of_val(&paso()));
}
