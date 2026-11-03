use std::sync::Mutex;
use std::thread;

fn main() {
    let tinta = Mutex::new(10);
    let papel = Mutex::new(10);
    thread::scope(|s| {
        s.spawn(|| {
            let mut t = tinta.lock().unwrap();
            let mut p = papel.lock().unwrap();
            *t -= 1;
            *p -= 1;
        });
        s.spawn(|| {
            let mut t = tinta.lock().unwrap();
            let mut p = papel.lock().unwrap();
            *t -= 2;
            *p -= 2;
        });
    });
    println!("tinta = {}, papel = {}", tinta.lock().unwrap(), papel.lock().unwrap());
}
