use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let datos = Arc::new(Mutex::new(vec![1, 2, 3]));

    let d = Arc::clone(&datos);
    let r = thread::spawn(move || {
        let mut g = d.lock().unwrap();
        g.push(4);
        panic!("algo salió mal a mitad de la actualización");
    })
    .join();
    println!("¿el hilo falló? {}", r.is_err());

    // El Mutex queda «envenenado»: lock() devuelve Err.
    match datos.lock() {
        Ok(g) => println!("todo bien: {g:?}"),
        Err(veneno) => {
            // Podemos recuperar el dato igualmente, sabiendo que puede estar a medias.
            let g = veneno.into_inner();
            println!("envenenado; el dato es {g:?}");
        }
    }
}
