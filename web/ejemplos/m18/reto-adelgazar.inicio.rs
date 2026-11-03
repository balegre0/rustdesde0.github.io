async fn paso() {}

// El array solo se usa ANTES del .await: enciérralo en un bloque
// para que no tenga que guardarse dentro del future.
async fn primero(x: u8) -> u8 {
    let tabla = [x; 100];
    let p = tabla[0];
    paso().await;
    p + tabla[1] - tabla[1]
}

fn main() {
    println!("{} bytes", std::mem::size_of_val(&primero(5)));
}
