use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 1. Tres productores: cada uno con su propio clon del emisor.
    for id in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for n in 0..2 {
                tx.send(format!("productor {id}, mensaje {n}")).unwrap();
            }
        });
    }
    // 2. Soltamos el emisor original: si no, el bucle de abajo no acabaría nunca.
    drop(tx);

    // 3. Un solo consumidor (la «sc» de mpsc).
    let mut total = 0;
    for msg in rx {
        println!("{msg}");
        total += 1;
    }
    println!("total: {total}");
}
