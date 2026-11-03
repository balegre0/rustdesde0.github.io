use std::sync::Mutex;
use std::thread;

fn main() {
    let contador = Mutex::new(0);
    thread::scope(|s| {
        s.spawn(|| *contador.lock().unwrap() += 1);
        s.spawn(|| *contador.lock().unwrap() += 1);
    });
    println!("{}", contador.into_inner().unwrap());
}
