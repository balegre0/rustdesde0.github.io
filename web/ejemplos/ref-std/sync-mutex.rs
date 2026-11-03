use std::sync::Mutex;

fn main() {
    let contador = Mutex::new(0);
    {
        // lock() bloquea y da un guard; al soltarlo se desbloquea
        let mut guard = contador.lock().unwrap();
        *guard += 1;
    }
    *contador.lock().unwrap() += 10;
    println!("{:?}", contador.lock().unwrap());
    // try_lock no espera: falla si ya está bloqueado
    let g = contador.lock().unwrap();
    println!("{}", contador.try_lock().is_err());
    drop(g);
    println!("{}", contador.into_inner().unwrap());
}
