// `{:p}` imprime una dirección de memoria; `&local` es «la dirección de `local`»
// (lo verás a fondo en el mundo 07).
fn nivel(n: u32) {
    // 1. Cada llamada tiene su propio marco, con su propia `local`
    let local: u64 = n as u64 * 100;
    println!("nivel {n}: local = {local:>3} vive en {:p}", &local);
    if n < 3 {
        nivel(n + 1);
    }
    // 3. Al volver, el marco de esta llamada se libera
    println!("salgo del nivel {n}");
}

fn main() {
    let en_main: u64 = 7;
    println!("main:    en_main = {en_main} vive en {:p}", &en_main);
    // 2. Cada llamada anidada apila un marco nuevo, en direcciones más bajas
    nivel(1);
}
