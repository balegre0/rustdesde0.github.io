use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    // (¿está listo?, condvar para avisar)
    let par = Arc::new((Mutex::new(false), Condvar::new()));

    let p = Arc::clone(&par);
    let cocinero = thread::spawn(move || {
        let (listo, cv) = &*p;
        *listo.lock().unwrap() = true; // 1. cambia el estado CON el candado
        cv.notify_one(); // 2. avisa a quien espera
    });

    let (listo, cv) = &*par;
    // 3. wait_while suelta el candado, duerme, y al despertar vuelve a
    //    comprobar la condición (por si fue un despertar falso).
    let g = cv.wait_while(listo.lock().unwrap(), |l| !*l).unwrap();
    println!("¡la comida está lista! ({})", *g);
    drop(g);
    cocinero.join().unwrap();
}
