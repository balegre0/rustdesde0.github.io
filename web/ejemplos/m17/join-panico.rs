use std::thread;

fn main() {
    let h = thread::spawn(|| {
        let v: Vec<i32> = Vec::new();
        v[3] // índice fuera de rango: este hilo entra en pánico
    });

    // join() devuelve Err si el hilo terminó con un pánico.
    match h.join() {
        Ok(x) => println!("el hilo devolvió {x}"),
        Err(_) => println!("el hilo entró en pánico, pero main sigue vivo"),
    }
    println!("main termina con normalidad");
}
