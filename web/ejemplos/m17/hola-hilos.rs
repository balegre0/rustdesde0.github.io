use std::thread;

fn main() {
    // 1. Lanzamos tres hilos. Cada uno calcula algo y lo DEVUELVE.
    let mut manijas = Vec::new();
    for id in 1..=3 {
        let h = thread::spawn(move || {
            println!("hilo {id}: trabajando");
            id * 100
        });
        manijas.push(h);
    }

    // 2. join() espera a que cada hilo termine y nos da su valor.
    let mut total = 0;
    for h in manijas {
        total += h.join().unwrap();
    }

    // 3. Esta línea siempre sale la última: ya hemos esperado a todos.
    println!("total = {total}");
}
