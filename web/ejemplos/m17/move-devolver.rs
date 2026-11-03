use std::thread;

fn main() {
    let mut lista = vec![5, 1, 4, 2, 3];

    // 1. Movemos la lista al hilo, que la ordena...
    let h = thread::spawn(move || {
        lista.sort();
        lista // 2. ...y la DEVUELVE: la propiedad vuelve a main.
    });

    // 3. join() nos entrega la lista ya ordenada.
    let lista = h.join().unwrap();
    println!("{lista:?}");
}
