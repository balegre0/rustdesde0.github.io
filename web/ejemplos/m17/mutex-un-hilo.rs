use std::sync::Mutex;

fn main() {
    // 1. El Mutex ENVUELVE el dato: no hay forma de tocarlo sin el candado.
    let saldo = Mutex::new(100);

    {
        // 2. lock() devuelve un guard: un préstamo exclusivo con candado.
        let mut g = saldo.lock().unwrap();
        *g -= 30;
        println!("dentro: {}", *g);

        // 3. Mientras el guard vive, nadie más puede tomar el candado.
        println!("¿libre? {}", saldo.try_lock().is_ok());
    } // 4. El guard se destruye aquí y el candado se suelta solo.

    println!("¿libre ahora? {}", saldo.try_lock().is_ok());
    println!("final: {}", saldo.into_inner().unwrap());
}
